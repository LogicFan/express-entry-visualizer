use std::{cmp::Ordering, collections::BinaryHeap};

use chrono::{Days, NaiveDate};

use super::calc;
use crate::data;

#[derive(Debug, Clone, Copy)]
struct RateModifier {
    expiry: NaiveDate,
    value: calc::ScorePool,
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
    _rate: calc::ScorePool,
}

impl RateAccumulator {
    fn new() -> Self {
        Self {
            _heap: BinaryHeap::new(),
            _rate: calc::ScorePool::zero(),
        }
    }

    fn rate(&self) -> calc::ScorePool {
        self._rate
    }

    fn next_expiry(&self) -> Option<NaiveDate> {
        self._heap.peek().map(|modifier| modifier.expiry)
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
    pub const SUBMIT_DAYS: usize = 15;

    pub fn pool_increase_rate(
        pool_data: &Vec<data::Pool>,
        invite_data: &Vec<data::Invite>,
    ) -> (Vec<NaiveDate>, Vec<calc::ScorePool>) {
        if pool_data.len() == 0 {
            return (Vec::new(), Vec::new());
        }

        let i_0 = pool_data.first().unwrap().date;
        let i_n = pool_data.last().unwrap().date + Days::new(1);

        let capacity = (i_n - i_0).num_days() as usize;
        let mut dates = Vec::with_capacity(capacity);
        let mut rates = Vec::with_capacity(capacity);

        let mut pools = pool_data.clone();
        let mut invites = invite_data
            .iter()
            .copied()
            .filter(|invite| invite.date >= i_0)
            .collect::<Vec<_>>();

        pools.reverse();
        invites.reverse();

        let mut rate_acc = RateAccumulator::new();
        let mut pool_invite = calc::ScorePool::zero(); // based on
                                                       // how date_0 is defined, this value will be immediately re-assigned.

        // there are two potential increase from the raw data
        // 1. pool-based increase: the number difference directly computed by
        // the pool data
        // 2. invite-based increase: after a person get his invitation, he will
        // be removed from the pool, so there must be another person to enter
        // the pool to keep the number equal.
        let mut i = i_0;
        while i != i_n {
            let mut i_next = i_n;
            rate_acc.update(i);

            // calculate the pool-based increase
            match pools.last().copied() {
                None => (),
                Some(pool_current) => {
                    assert!(pool_current.date >= i);
                    if pool_current.date == i {
                        pool_invite = pool_current.into(); // recorded for the following invite-based increase computation
                        pools.pop();
                        match pools.last().copied() {
                            None => (),
                            Some(pool_next) => {
                                assert!(pool_next.date > pool_current.date);
                                let expiry = pool_next.date;

                                let days = (pool_next.date - i).num_days() as f64;
                                let pool0: calc::ScorePool = pool_current.into();
                                let pool1: calc::ScorePool = pool_next.into();
                                rate_acc.insert(RateModifier {
                                    value: (pool1 - pool0) / days,
                                    expiry,
                                });

                                // potential next i: next pool data date
                                i_next = std::cmp::min(i_next, pool_next.date);
                            }
                        }
                    } else {
                        // potential next i: next pool data date
                        i_next = std::cmp::min(i_next, pool_current.date);
                    }
                }
            }

            // calculate the invite-based increase
            while let Some(invite) = invites.last() {
                assert!(invite.date >= i);
                if invite.date != i {
                    // potential next i: next invite data date
                    i_next = std::cmp::min(i_next, invite.date);
                    break;
                }

                let invited = pool_invite.invite(invite);
                rate_acc.insert(RateModifier {
                    value: invited / Self::SUBMIT_DAYS as f64,
                    expiry: i + Days::new(Self::SUBMIT_DAYS as u64),
                });
                pool_invite = pool_invite - invited; // remove already invited
                                                     // candidates from the pool to avoid duplicate counts.

                invites.pop();
            }

            match rate_acc.next_expiry() {
                None => (),
                Some(expiry_date) => {
                    // potential next i: next rate accumulator expiry date
                    i_next = std::cmp::min(i_next, expiry_date);
                }
            }

            if i > i_0 + Days::new(Self::SUBMIT_DAYS as u64) {
                // ignore first 60 days since they are under estimated.

                let interval = (i_next - i).num_days();
                dates.push(i + Days::new(((interval + 1) / 2) as u64)); // use the mid-point
                rates.push(rate_acc.rate());
            }

            i = i_next;
        }

        (dates, rates)
    }

    pub fn projected_rate(rate_data: &Vec<calc::ScorePool>) -> calc::ScorePool {
        const PAST_DAYS: usize = 181;
        rate_data
            .iter()
            .copied()
            .rev()
            .take(PAST_DAYS)
            .reduce(|x, y| x + y)
            .unwrap_or(calc::ScorePool::zero())
            / PAST_DAYS as f64
    }
}
