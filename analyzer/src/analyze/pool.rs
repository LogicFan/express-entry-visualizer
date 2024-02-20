use chrono::Datelike;
use std::ops::{Add, Div, Index, IndexMut, Mul, Sub};

use crate::data::{self, Category};

#[derive(Debug, Clone, Copy)]
pub struct Pool([f64; data::Pool::N]);

impl Pool {
    const N: usize = data::Pool::N;

    pub fn total(&self) -> f64 {
        self.0.iter().sum()
    }

    pub fn count(&self, i: usize) -> f64 {
        self[i]
    }

    pub fn min_score(i: usize) -> i64 {
        data::Pool::min_score(i)
    }

    pub fn max_score(i: usize) -> i64 {
        data::Pool::max_score(i)
    }
}

impl Default for Pool {
    fn default() -> Self {
        Self([0_f64; Self::N])
    }
}

impl From<data::Pool> for Pool {
    fn from(value: data::Pool) -> Self {
        Self(value.data)
    }
}

impl Index<usize> for Pool {
    type Output = f64;
    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl IndexMut<usize> for Pool {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.0[i]
    }
}

impl Add for Pool {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        for i in 0..Pool::N {
            self[i] = self[i] + rhs[i];
        }
        self
    }
}

impl Sub for Pool {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        for i in 0..Pool::N {
            self[i] = self[i] - rhs[i];
        }
        self
    }
}

impl Mul for Pool {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        for i in 0..Pool::N {
            self[i] = self[i] * rhs[i];
        }
        self
    }
}

impl Div for Pool {
    type Output = Self;

    fn div(mut self, rhs: Self) -> Self::Output {
        for i in 0..Pool::N {
            self[i] = self[i] / rhs[i];
        }
        self
    }
}

impl Mul<f64> for Pool {
    type Output = Self;

    fn mul(mut self, rhs: f64) -> Self::Output {
        for i in 0..Pool::N {
            self[i] = self[i] * rhs;
        }
        self
    }
}

impl Div<f64> for Pool {
    type Output = Self;

    fn div(mut self, rhs: f64) -> Self::Output {
        for i in 0..Pool::N {
            self[i] = self[i] / rhs;
        }
        self
    }
}

impl Pool {
    fn multiplier_invite_general(&self, mut count: f64) -> Self {
        let mut multiplier = Self::default();

        for i in (0..Self::N).rev() {
            if count > self[i] {
                multiplier[i] = 1.0;
                count -= self[i]
            } else {
                multiplier[i] = count / self[i];
                // count = 0.0;
                break;
            }
        }

        multiplier
    }

    fn multiplier_within_score(&self, min_score: f64, max_score: f64) -> Self {
        let mut multiplier = Self::default();

        for i in (0..Self::N).rev() {
            let min_d = Pool::min_score(i) as f64;
            let max_d = Pool::max_score(i) as f64;

            let min_n = f64::min(max_d, f64::max(min_score, min_d));
            let max_n = f64::max(min_d, f64::min(max_score, max_d));

            multiplier[i] = (max_n - min_n) / (max_d - min_d);
        }

        multiplier
    }
}

impl Pool {
    pub fn non_pnp(mut self) -> Self {
        self[14] = 0_f64; // 14-th bucket is 601 to 1200
        self
    }

    pub fn invite(mut self, invitation: &data::Invite) -> Self {
        if !invitation.pathway.is_pnp() {
            self = self.non_pnp();
        }

        if invitation.category == Category::General {
            let m1 = self.multiplier_invite_general(invitation.size as f64);
            self = self * m1;
        } else {
            let m1 = self.multiplier_within_score(invitation.score as f64, 1200.0);
            self = self * m1;
            let m2 = invitation.size as f64 / self.total();
            self = self * m2;
        }

        self
    }
}

// pub trait PoolAnalyze {
//     fn count_above(&self, score: i64) -> f64;
//     fn count_within(&self, min: i64, max: i64) -> f64 {
//         self.count_above(min) - self.count_above(max)
//     }
//     fn invite(&mut self, invitation: &Invitation);
//     fn increment(&self, other: &Pool, invitations: &Vec<Invitation>)
//         -> [f64; Pool::SUB_POOL_COUNT];
// }

// impl PoolAnalyze for Pool {
//     fn count_above(&self, score: i64) -> f64 {
//         let mut count = 0_f64;
//         for i in 0..Pool::N {
//             let sub_pool = &self.data[i];

//             if sub_pool.min_score > score {
//                 count += sub_pool.population as f64;
//             } else if sub_pool.max_score >= score {
//                 let pct = (sub_pool.max_score - score) as f64
//                     / (sub_pool.max_score - sub_pool.min_score) as f64;

//                 count += sub_pool.population as f64 * pct
//             }
//         }
//         count
//     }

//     fn invite(&mut self, invitation: &Invitation) {
//         if invitation.category == Category::General {
//             let mut count = invitation.size;
//             for i in (0..Pool::N).rev() {
//                 if count > self.data[i].population {
//                     count -= self.data[i].population;
//                     self.data[i].population = 0;
//                 } else {
//                     self.data[i].population -= count;
//                     count = 0;
//                     break;
//                 }
//             }
//         } else {
//             let count = invitation.size;
//             let score = invitation.score;
//             let general_count = self.count_above(score);
//             let pct = count as f64 / general_count;

//             for i in 0..Pool::N {
//                 let sub_pool = &mut self.data[i];
//                 if sub_pool.min_score > score {
//                     sub_pool.population =
//                         f64::round(sub_pool.population as f64 - sub_pool.population as f64 * pct)
//                             as i64;
//                 } else if sub_pool.max_score >= score {
//                     let sub_pool_pct = (sub_pool.max_score - score) as f64
//                         / (sub_pool.max_score - sub_pool.min_score) as f64;

//                     sub_pool.population = f64::round(
//                         sub_pool.population as f64
//                             - sub_pool.population as f64 * pct * sub_pool_pct,
//                     ) as i64;
//                 }
//             }
//         }
//     }

//     fn increment(
//         &self,
//         other: &Pool,
//         invitations: &Vec<Invitation>,
//     ) -> [f64; Pool::SUB_POOL_COUNT] {
//         let mut pool_after_invite = *self;
//         for invitation in invitations {
//             if self.date <= invitation.date && invitation.date < other.date {
//                 pool_after_invite.invite(invitation);
//             }
//         }

//         let mut res = [0_f64; Pool::SUB_POOL_COUNT];
//         let days_between = other.date.signed_duration_since(self.date).num_days() as f64;
//         pool_after_invite
//             .data
//             .iter()
//             .zip(other.data.iter())
//             .map(|(before, after)| (after.population - before.population) as f64 / days_between)
//             .enumerate()
//             .for_each(|(i, v)| res[i] = v);
//         res
//     }
// }

#[cfg(test)]
mod tests {
    use super::Pool;
    use tokio;

    #[tokio::test]
    async fn non_pnp() {
        let pool = Pool([
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
        ]);

        let res0 = pool.non_pnp();
        assert_eq!(
            res0.0,
            [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 0.0]
        );
    }

    #[tokio::test]
    async fn multiplier_invite_general() {
        let pool = Pool([
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
        ]);

        let res0 = pool.multiplier_invite_general(69.0) * pool;
        assert_eq!(
            res0.0,
            [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 4.0, 11.0, 12.0, 13.0, 14.0, 15.0]
        );

        let res1 = pool.non_pnp().multiplier_invite_general(69.0) * pool.non_pnp();
        assert_eq!(
            res1.0,
            [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 0.0]
        );
    }

    #[tokio::test]
    async fn multiplier_within_score() {
        let pool = Pool([
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
        ]);

        let res0 = pool.multiplier_within_score(0.0, 1200.0) * pool;
        assert_eq!(
            res0.0,
            [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0]
        );

        let res1 = pool.multiplier_within_score(466.0, 1200.0) * pool;
        assert_eq!(
            res1.0,
            [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 4.0, 11.0, 12.0, 13.0, 14.0, 15.0]
        );

        let res2 = pool.multiplier_within_score(0.0, 466.0) * pool;
        assert_eq!(
            res2.0,
            [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 6.0, 0.0, 0.0, 0.0, 0.0, 0.0]
        );

        let res3 = pool.multiplier_within_score(414.0, 466.0) * pool;
        assert_eq!(
            res3.0,
            [0.0, 0.0, 0.0, 0.0, 3.0, 6.0, 7.0, 8.0, 9.0, 6.0, 0.0, 0.0, 0.0, 0.0, 0.0]
        );
    }
}
