use crate::data::{self, CategoryCode};
use std::ops::{Add, Div, Index, IndexMut, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ScorePool([f64; data::Pool::N]);

impl ScorePool {
    const N: usize = data::Pool::N;

    pub fn zero() -> Self {
        Self::default()
    }

    pub fn total(&self) -> f64 {
        self.0.iter().sum()
    }

    pub fn min_score(i: usize) -> i64 {
        data::Pool::min_score(i)
    }

    pub fn max_score(i: usize) -> i64 {
        data::Pool::max_score(i)
    }
}

impl Default for ScorePool {
    fn default() -> Self {
        Self([0_f64; Self::N])
    }
}

impl From<data::Pool> for ScorePool {
    fn from(value: data::Pool) -> Self {
        Self(value.data)
    }
}

impl Index<usize> for ScorePool {
    type Output = f64;
    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl IndexMut<usize> for ScorePool {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.0[i]
    }
}

impl Add for ScorePool {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        for i in 0..ScorePool::N {
            self[i] = self[i] + rhs[i];
        }
        self
    }
}

impl Sub for ScorePool {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        for i in 0..ScorePool::N {
            self[i] = self[i] - rhs[i];
        }
        self
    }
}

impl Mul for ScorePool {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        for i in 0..ScorePool::N {
            self[i] = self[i] * rhs[i];
        }
        self
    }
}

impl Div for ScorePool {
    type Output = Self;

    fn div(mut self, rhs: Self) -> Self::Output {
        for i in 0..ScorePool::N {
            self[i] = self[i] / rhs[i];
        }
        self
    }
}

impl Mul<f64> for ScorePool {
    type Output = Self;

    fn mul(mut self, rhs: f64) -> Self::Output {
        for i in 0..ScorePool::N {
            self[i] = self[i] * rhs;
        }
        self
    }
}

impl Div<f64> for ScorePool {
    type Output = Self;

    fn div(mut self, rhs: f64) -> Self::Output {
        for i in 0..ScorePool::N {
            self[i] = self[i] / rhs;
        }
        self
    }
}

impl ScorePool {
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
            let min_d = ScorePool::min_score(i) as f64;
            let max_d = ScorePool::max_score(i) as f64;

            let min_n = f64::min(max_d, f64::max(min_score, min_d));
            let max_n = f64::max(min_d, f64::min(max_score, max_d));

            multiplier[i] = (max_n - min_n) / (max_d - min_d);
        }

        multiplier
    }
}

impl ScorePool {
    pub fn pnp(self) -> Self {
        let mut pool = Self::default();
        pool[14] = self[14]; // 14-th bucket is 601 to 1200
        pool
    }

    pub fn non_pnp(self) -> Self {
        self - self.pnp()
    }

    pub fn invite(mut self, _invite: &data::Invite) -> Self {
        if !_invite.pathway.is_pnp() {
            self = self.non_pnp();
        }

        if _invite.category.code == CategoryCode::General {
            let m1 = self.multiplier_invite_general(_invite.size as f64);
            self = self * m1;
        } else {
            let m1 = self.multiplier_within_score(_invite.score as f64, 1200.0);
            self = self * m1;
            let m2 = _invite.size as f64 / self.total();
            self = self * m2;
        }

        self
    }

    pub fn within_score(self, min_score: f64, max_score: f64) -> Self {
        let multiplier = self.multiplier_within_score(min_score, max_score);
        self * multiplier
    }
}

#[cfg(test)]
mod tests {
    use super::ScorePool;

    #[tokio::test]
    async fn non_pnp() {
        let pool = ScorePool([
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
        let pool = ScorePool([
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
        let pool = ScorePool([
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
