use std::{cmp::Ordering, collections::BinaryHeap};

use chrono::{Days, NaiveDate};

use super::calc;
use crate::data;

#[derive(Debug, Clone, Copy)]
struct RateModifier {
    expiry: NaiveDate,
    value: calc::Pool,
}
impl PartialEq for RateModifier {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Eq for RateModifier {}
impl PartialOrd for RateModifier {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for RateModifier {
    fn cmp(&self, other: &Self) -> Ordering {
        self.expiry.cmp(&other.expiry).reverse()
    }
}

struct RateAccumulator {
    _heap: BinaryHeap<RateModifier>,
    _rate: calc::Pool,
}

impl RateAccumulator {
    fn new() -> Self {
        Self {
            _heap: BinaryHeap::new(),
            _rate: calc::Pool::zero(),
        }
    }

    fn rate(&self) -> calc::Pool {
        self._rate
    }

    fn insert(&mut self, modifier: RateModifier) {
        self._rate = self._rate + modifier.value;
        self._heap.push(modifier)
    }

    fn update(&mut self, date: NaiveDate) {
        while let Some(modifier) = self._heap.peek() {
            assert!(modifier.expiry >= date);
            if modifier.expiry != date {
                break;
            }
            self._rate = self._rate - modifier.value;
            self._heap.pop();
        }
    }
}

pub struct RateAnalyzer;

impl RateAnalyzer {
    pub const SUBMIT_DAYS: usize = 60;

    pub fn pool_increase_rate(
        pool_data: &Vec<data::Pool>,
        invite_data: &Vec<data::Invite>,
    ) -> (Vec<NaiveDate>, Vec<calc::Pool>) {
        if pool_data.len() == 0 {
            return (Vec::new(), Vec::new());
        }

        let date_0 = pool_data.first().unwrap().date;
        let date_n = pool_data.last().unwrap().date + Days::new(1);

        let capacity = (date_n - date_0).num_days() as usize;
        let mut dates = Vec::with_capacity(capacity);
        let mut rates = Vec::with_capacity(capacity);

        let mut pools = pool_data.clone();
        let mut invites = invite_data
            .iter()
            .copied()
            .filter(|invite| invite.date >= date_0)
            .collect::<Vec<_>>();

        pools.reverse();
        invites.reverse();

        let mut rate_acc = RateAccumulator::new();
        let mut invite_pool = calc::Pool::zero(); // based on how date_0 is defined, this value will be immediately re-assigned.

        let mut date = date_0;
        while date != date_n {
            rate_acc.update(date);

            match pools.last().copied() {
                None => (),
                Some(current_pool) => {
                    if current_pool.date == date {
                        invite_pool = current_pool.into();
                        pools.pop();
                        match pools.last().copied() {
                            None => (),
                            Some(next_pool) => {
                                assert!(next_pool.date > date);
                                let expiry = next_pool.date;

                                // based on how pool are constructed, next_pool.date > current_pool.date
                                let days = (next_pool.date - date).num_days() as f64;
                                let pool0: calc::Pool = invite_pool;
                                let pool1: calc::Pool = next_pool.into();
                                rate_acc.insert(RateModifier {
                                    value: (pool1 - pool0) / days,
                                    expiry,
                                });
                            }
                        }
                    }
                }
            }

            while let Some(invite) = invites.last() {
                assert!(invite.date >= date);
                if invite.date != date {
                    break;
                }

                let invited = invite_pool.invite(invite);
                rate_acc.insert(RateModifier {
                    value: invited / Self::SUBMIT_DAYS as f64,
                    expiry: date + Days::new(Self::SUBMIT_DAYS as u64),
                });
                invite_pool = invite_pool - invited; // remove already invited candidates to avoid duplicate counts.
                invites.pop();
            }

            if date > date_0 + Days::new(Self::SUBMIT_DAYS as u64) {
                // ignore first 60 days since they are under estimated.
                dates.push(date);
                rates.push(rate_acc.rate());
            }
            date = date + Days::new(1);
        }

        (dates, rates)
    }

    pub fn projected_rate(rate_data: &Vec<calc::Pool>) -> calc::Pool {
        const PAST_DAYS: usize = 181;
        rate_data
            .iter()
            .copied()
            .rev()
            .take(PAST_DAYS)
            .reduce(|x, y| x + y)
            .unwrap_or(calc::Pool::zero())
            / PAST_DAYS as f64
    }
}
