pub struct Smoother;
use std::ops::{Add, Mul};
use chrono::NaiveDate;
use super::calc::ScorePool;

pub(crate) trait SmoothLabel: Copy {
    fn steps(self, other: Self) -> i64;
}

pub(crate) trait SmoothValue: Add<Output = Self> + Mul<f64, Output = Self> + Copy {}

impl SmoothLabel for NaiveDate {
    fn steps(self, other: Self) -> i64 {
        (self - other).num_days()
    }
}

impl SmoothValue for ScorePool {}

impl Smoother {
    pub fn exponential<L, D>(labels: &Vec<L>, values: &mut Vec<D>, alpha: f64)
    where
        L: SmoothLabel,
        D: SmoothValue,
    {
        assert!(labels.len() == values.len());
        if labels.is_empty() {
            return;
        }

        let mut value_prev = values[0];
        for k in 1..values.len() {
            let steps = labels[k].steps(labels[k - 1]);

            let a = alpha;
            let b = (1.0 - alpha).powi(steps as i32);
            let r = 1.0 - a - b;
            assert!(a + b + r == 1.0);

            let tmp_value = values[k];
            values[k] = values[k] * a + values[k - 1] * b + value_prev * r;
            value_prev = tmp_value;
        }
    }
}
