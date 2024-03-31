use crate::data::CategoryCode;
use std::ops::{Add, Div, Index, IndexMut, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CategoryPool([f64; CategoryCode::N]);

impl CategoryPool {
    const N: usize = CategoryCode::N;

    pub fn zero() -> Self {
        Self::default()
    }

    pub fn total(&self) -> f64 {
        self.0.iter().sum()
    }
}

impl Default for CategoryPool {
    fn default() -> Self {
        Self([0_f64; Self::N])
    }
}

impl Index<CategoryCode> for CategoryPool {
    type Output = f64;
    fn index(&self, i: CategoryCode) -> &Self::Output {
        &self.0[i as usize]
    }
}

impl IndexMut<CategoryCode> for CategoryPool {
    fn index_mut(&mut self, i: CategoryCode) -> &mut Self::Output {
        &mut self.0[i as usize]
    }
}

impl Index<usize> for CategoryPool {
    type Output = f64;
    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl Add for CategoryPool {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        for i in CategoryCode::values().iter().copied() {
            self[i] = self[i] + rhs[i];
        }
        self
    }
}

impl Sub for CategoryPool {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        for i in CategoryCode::values().iter().copied() {
            self[i] = self[i] - rhs[i];
        }
        self
    }
}

impl Mul for CategoryPool {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        for i in CategoryCode::values().iter().copied() {
            self[i] = self[i] * rhs[i];
        }
        self
    }
}

impl Div for CategoryPool {
    type Output = Self;

    fn div(mut self, rhs: Self) -> Self::Output {
        for i in CategoryCode::values().iter().copied() {
            self[i] = self[i] / rhs[i];
        }
        self
    }
}

impl Mul<f64> for CategoryPool {
    type Output = Self;

    fn mul(mut self, rhs: f64) -> Self::Output {
        for i in CategoryCode::values().iter().copied() {
            self[i] = self[i] * rhs;
        }
        self
    }
}

impl Div<f64> for CategoryPool {
    type Output = Self;

    fn div(mut self, rhs: f64) -> Self::Output {
        for i in CategoryCode::values().iter().copied() {
            self[i] = self[i] / rhs;
        }
        self
    }
}

impl CategoryPool {
    pub fn normalize(self) -> Self {
        if self.total() == 0.0 {
            Self::default()
        } else {
            self / self.total()
        }
    }
}
