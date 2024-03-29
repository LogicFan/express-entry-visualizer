use super::raw::{EeRounds123En, RawData};
use super::utils::{parse_date, parse_i32};
use chrono::NaiveDate;
use itertools::Itertools;
use std::cmp::Ordering;
use wasm_bindgen::throw_str;

#[derive(Debug, Clone, Copy)]
pub struct Pool {
    pub data: [f64; Pool::N],
    pub date: NaiveDate,
}

impl PartialEq for Pool {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Pool {}

impl PartialOrd for Pool {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pool {
    fn cmp(&self, other: &Self) -> Ordering {
        self.date.cmp(&other.date)
    }
}

impl Pool {
    pub const N: usize = 15;

    pub fn parse(raw_data: &RawData) -> Self {
        Self {
            date: parse_date(&raw_data.draw_distribution_as_on),
            data: [
                parse_i32(&raw_data.dd17) as f64,
                parse_i32(&raw_data.dd16) as f64,
                parse_i32(&raw_data.dd15) as f64,
                parse_i32(&raw_data.dd14) as f64,
                parse_i32(&raw_data.dd13) as f64,
                parse_i32(&raw_data.dd12) as f64,
                parse_i32(&raw_data.dd11) as f64,
                parse_i32(&raw_data.dd10) as f64,
                parse_i32(&raw_data.dd8) as f64,
                parse_i32(&raw_data.dd7) as f64,
                parse_i32(&raw_data.dd6) as f64,
                parse_i32(&raw_data.dd5) as f64,
                parse_i32(&raw_data.dd4) as f64,
                parse_i32(&raw_data.dd2) as f64,
                parse_i32(&raw_data.dd1) as f64,
            ],
        }
    }

    // ensure sorted
    pub fn parse_all(raw_data: &EeRounds123En) -> Vec<Self> {
        raw_data
            .rounds
            .iter()
            .map(|round| Self::parse(round))
            .filter(|pool| pool.is_valid())
            .sorted()
            .group_by(|pool| pool.date)
            .into_iter()
            .map(|(_, mut group)| group.nth(0).unwrap())
            .collect::<Vec<_>>()
    }

    pub fn is_valid(&self) -> bool {
        self.total() != 0_f64
    }

    pub fn total(&self) -> f64 {
        self.data.iter().sum()
    }

    pub fn count(&self, i: usize) -> f64 {
        self.data[i]
    }

    pub fn min_score(i: usize) -> i64 {
        match i {
            0 => 0,
            1 => 300,
            2 => 350,
            3 => 400,
            4 => 410,
            5 => 420,
            6 => 430,
            7 => 440,
            8 => 450,
            9 => 460,
            10 => 470,
            11 => 480,
            12 => 490,
            13 => 500,
            14 => 600,
            _ => throw_str("index out of bound"),
        }
    }

    pub fn max_score(i: usize) -> i64 {
        match i {
            0 => 300,
            1 => 350,
            2 => 400,
            3 => 410,
            4 => 420,
            5 => 430,
            6 => 440,
            7 => 450,
            8 => 460,
            9 => 470,
            10 => 480,
            11 => 490,
            12 => 500,
            13 => 600,
            14 => 1200,
            _ => throw_str("index out of bound"),
        }
    }

    pub fn as_color(i: usize) -> String {
        let p = i as f64 / 15_f64;
        const P: f64 = 0.6;

        let r: i32;
        let g: i32;

        if p < P {
            r = 255;
            g = f64::floor((p / P) * 255.0) as i32;
        } else {
            r = 255 - f64::floor(((p - P) / (1.0 - P)) * 255.0) as i32;
            g = 255;
        }
        format!("#{:02x}{:02x}00", r, g)
    }

    pub fn as_str(i: usize) -> String {
        format!(
            "{} - {}",
            if Pool::min_score(i) == 0 {
                0
            } else {
                Pool::min_score(i) + 1
            },
            Pool::max_score(i)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::super::raw::raw_data;
    use super::Pool;

    #[tokio::test]
    async fn parse_data() {
        let x = raw_data().await;
        let p = Pool::parse_all(&x);

        assert!(p.iter().all(|x| x.is_valid()));
    }

    #[tokio::test]
    async fn color() {
        assert_eq!("#ff8d00", Pool::as_color(5))
    }
}
