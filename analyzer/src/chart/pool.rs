use super::dataset::PointStyle;
use crate::analyze::rate::RateAnalyzer;
use crate::analyze::smooth::ExponentialSmoothing;
use crate::chart::dataset::{ChartData, LineDataset, Tooltip};
use crate::chart::utils::{ToTimestamp, SERIALIZER};
use crate::data::{Invite, Pool};
use chrono::Days;
use serde::Serialize;
use std::iter;
use std::ops::{Index, Mul};
use wasm_bindgen::prelude::*;

struct PoolAcc<T>
where
    T: Index<usize, Output = f64>,
{
    data: T,
}

impl<T> PoolAcc<T>
where
    T: Index<usize, Output = f64>,
{
    const N: usize = Pool::N;

    pub fn new(data: T) -> Self {
        Self { data }
    }

    pub fn sum(&self, i: usize) -> f64 {
        (i..Self::N).map(|k| self.data[k]).sum()
    }
}

#[wasm_bindgen]
pub fn wasm_pool_n() -> JsValue {
    Pool::N.serialize(&SERIALIZER).unwrap_throw()
}

#[wasm_bindgen]
pub fn wasm_pool_count_data(pool_data: *const Vec<Pool>) -> JsValue {
    let pool_data = unsafe { pool_data.as_ref().unwrap_throw() };
    let labels = pool_data
        .iter()
        .map(|pool| pool.date.to_timestamp() as f64)
        .collect::<Vec<_>>();
    let datasets = (0..Pool::N)
        .rev()
        .into_iter()
        .map(|i| {
            let data = pool_data
                .iter()
                .map(|pool| Some(PoolAcc::new(*pool).sum(i)))
                .collect::<Vec<_>>();

            LineDataset {
                label: format!("{} - {}", Pool::min_score(i), Pool::max_score(i)),
                data,
                background_color: Pool::as_color(i),
                border_color: Pool::as_color(i),
                fill: true,
                point_style: PointStyle(None),
                ..Default::default()
            }
        })
        .collect::<Vec<_>>();

    ChartData {
        labels,
        datasets,
        tooltip: Tooltip {
            title: None,
            label: None,
        },
    }
    .serialize(&SERIALIZER)
    .unwrap_throw()
}

#[wasm_bindgen]
pub fn wasm_pool_count_y_max(pool_data: *const Vec<Pool>) -> JsValue {
    let pool_date = unsafe { pool_data.as_ref().unwrap_throw() };
    pool_date
        .iter()
        .map(|pool| pool.total())
        .max_by(|a, b| a.total_cmp(b))
        .unwrap_or(0.0)
        .mul(1.1)
        .serialize(&SERIALIZER)
        .unwrap_throw()
}

#[wasm_bindgen]
pub fn wasm_pool_count_x_min(pool_data: *const Vec<Pool>) -> JsValue {
    let pool_data = unsafe { pool_data.as_ref().unwrap_throw() };
    pool_data
        .first()
        .map(|pool| pool.date.to_timestamp() as f64)
        .unwrap_or(0.0)
        .serialize(&SERIALIZER)
        .unwrap_throw()
}

#[wasm_bindgen]
pub fn wasm_pool_count_x_max(pool_data: *const Vec<Pool>) -> JsValue {
    let pool_data = unsafe { pool_data.as_ref().unwrap_throw() };
    pool_data
        .last()
        .map(|pool| pool.date.to_timestamp() as f64)
        .unwrap_or(0.0)
        .serialize(&SERIALIZER)
        .unwrap_throw()
}

#[wasm_bindgen]
pub fn wasm_pool_rate_data(
    pool_data: *const Vec<Pool>,
    invite_data: *const Vec<Invite>,
) -> JsValue {
    let pool_data = unsafe { pool_data.as_ref().unwrap_throw() };
    let invite_data = unsafe { invite_data.as_ref().unwrap_throw() };

    let (mut rate_labels, mut rate_data) = RateAnalyzer::pool_increase_rate(pool_data, invite_data);
    let projected_rate = RateAnalyzer::projected_rate(&rate_data);
    ExponentialSmoothing::smooth(&mut rate_data, 0.03278688524);

    // insert projected_rate
    rate_labels.push(*rate_labels.last().unwrap() + Days::new(1));
    rate_labels.push(*rate_labels.last().unwrap() + Days::new(120));

    let labels = rate_labels
        .iter()
        .map(|date| date.to_timestamp() as f64)
        .collect::<Vec<_>>();

    let actual = (0..Pool::N).into_iter().rev().map(|i| {
        let data = rate_data
            .iter()
            .map(|rate| Some(PoolAcc::new(*rate).sum(i)))
            .chain(iter::repeat(None).take(2))
            .collect::<Vec<_>>();

        LineDataset {
            label: format!("> {}", Pool::min_score(i)),
            data,
            background_color: Pool::as_color(i),
            border_color: Pool::as_color(i),
            point_style: PointStyle(None),
            ..Default::default()
        }
    });

    let predict = (0..Pool::N).into_iter().rev().map(|i| {
        let data = iter::repeat(None)
            .take(rate_data.len())
            .chain(iter::repeat(Some(PoolAcc::new(projected_rate).sum(i))).take(2))
            .collect::<Vec<_>>();

        LineDataset {
            label: "none".into(),
            data,
            background_color: Pool::as_color(i),
            border_color: Pool::as_color(i),
            border_dash: [5.0, 5.0],
            point_style: PointStyle(None),
            ..Default::default()
        }
    });

    let datasets = actual.chain(predict).collect::<Vec<_>>();

    ChartData {
        labels,
        datasets,
        tooltip: Tooltip {
            title: None,
            label: Some(
                (0..Pool::N)
                    .rev()
                    .map(|i| format!("{}: {:.3}", Pool::as_str(i), projected_rate[i]))
                    .collect(),
            ),
        },
    }
    .serialize(&SERIALIZER)
    .unwrap_throw()
}

#[wasm_bindgen]
pub fn wasm_pool_rate_x_min(pool_data: *const Vec<Pool>) -> JsValue {
    let pool_data = unsafe { pool_data.as_ref().unwrap_throw() };
    pool_data
        .first()
        .map(|pool| (pool.date + Days::new(RateAnalyzer::SUBMIT_DAYS as u64)).to_timestamp() as f64)
        .unwrap_or(0.0)
        .serialize(&SERIALIZER)
        .unwrap_throw()
}

#[wasm_bindgen]
pub fn wasm_pool_rate_x_max(pool_data: *const Vec<Pool>) -> JsValue {
    let pool_data = unsafe { pool_data.as_ref().unwrap_throw() };
    pool_data
        .last()
        .map(|pool| (pool.date + Days::new(120)).to_timestamp() as f64)
        .unwrap_or(0.0)
        .serialize(&SERIALIZER)
        .unwrap_throw()
}
