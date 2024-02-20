use chrono::NaiveDate;
use itertools::Itertools;
use wasm_bindgen::throw_str;

use super::raw::{EeRounds123En, RawData};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Pool {
    pub data: [f64; Pool::N],
    pub date: NaiveDate,
}

fn parse_f64(x: &str) -> f64 {
    x.replace(",", "").parse().unwrap_or(0.0)
}

fn parse_date(x: &str) -> NaiveDate {
    NaiveDate::parse_from_str(x, "%B %d, %Y").unwrap_or(NaiveDate::MIN)
}

impl Pool {
    pub const N: usize = 15;

    pub fn parse(raw_data: &RawData) -> Self {
        Self {
            date: parse_date(&raw_data.draw_distribution_as_on),
            data: [
                parse_f64(&raw_data.dd17),
                parse_f64(&raw_data.dd16),
                parse_f64(&raw_data.dd15),
                parse_f64(&raw_data.dd14),
                parse_f64(&raw_data.dd13),
                parse_f64(&raw_data.dd12),
                parse_f64(&raw_data.dd11),
                parse_f64(&raw_data.dd10),
                parse_f64(&raw_data.dd8),
                parse_f64(&raw_data.dd7),
                parse_f64(&raw_data.dd6),
                parse_f64(&raw_data.dd5),
                parse_f64(&raw_data.dd4),
                parse_f64(&raw_data.dd2),
                parse_f64(&raw_data.dd1),
            ],
        }
    }

    pub fn parse_all(raw_data: &EeRounds123En) -> Vec<Self> {
        raw_data
            .rounds
            .iter()
            .map(|round| Self::parse(round))
            .filter(|pool| pool.is_valid())
            .group_by(|pool| pool.date)
            .into_iter()
            .map(|(_, mut group)| group.nth(0).unwrap())
            .sorted_by_key(|pool| pool.date)
            .collect()
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

        println!("{:?}", p);
    }

    #[tokio::test]
    async fn color() {
        println!("{}", Pool::as_color(5))
    }
}
