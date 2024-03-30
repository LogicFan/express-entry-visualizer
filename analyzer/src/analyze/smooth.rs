pub struct ExponentialSmoothing;
use std::ops::{Add, Mul};

impl ExponentialSmoothing {
    // TODO: support ata of variable length
    pub fn smooth<T>(data: &mut Vec<T>, alpha: f64)
    where
        T: Add<Output = T> + Mul<f64, Output = T> + Copy,
    {
        for i in 1..data.len() {
            data[i] = data[i] * alpha + data[i - 1] * (1.0 - alpha);
        }
    }
}
