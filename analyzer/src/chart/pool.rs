use chrono::Months;
use serde::Serialize;
use wasm_bindgen::prelude::*;

use crate::analyze::rate::RateAnalyzer;
use crate::chart::dataset::{ChartData, LineDataset, Tooltip};
use crate::chart::utils::{ToTimestamp, SERIALIZER};
use crate::data::{Invite, Pool};

use super::dataset::PointStyle;

#[wasm_bindgen]
pub fn wasm_pool_count_data(pool_data: *const Vec<Pool>) -> JsValue {
    let pool_data = unsafe { pool_data.as_ref().unwrap_throw() };
    let labels = pool_data
        .iter()
        .map(|pool| pool.date.to_timestamp() as f64)
        .collect::<Vec<_>>();
    let datasets = (0..Pool::N)
        .into_iter()
        .map(|i| {
            let data = pool_data
                .iter()
                .map(|pool| Some(pool.data[i]))
                .collect::<Vec<_>>();

            LineDataset {
                label: format!("{} - {}", Pool::min_score(i), Pool::max_score(i),),
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

    let rate_data = RateAnalyzer::pool_increase_rate(pool_data, invite_data);

    let labels = rate_data
        .iter()
        .map(|rate| rate.date.to_timestamp() as f64)
        .collect::<Vec<_>>();

    let datasets = (0..Pool::N)
        .into_iter()
        .map(|i| {
            let data = rate_data
                .iter()
                .map(|rate| Some(rate.data[i]))
                .collect::<Vec<_>>();

            LineDataset {
                label: format!("{} - {}", Pool::min_score(i), Pool::max_score(i),),
                data,
                background_color: Pool::as_color(i),
                border_color: Pool::as_color(i),
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
pub fn wasm_pool_x_min(pool_data: *const Vec<Pool>) -> JsValue {
    let pool_data = unsafe { pool_data.as_ref().unwrap_throw() };
    pool_data
        .first()
        .map(|pool| (pool.date - Months::new(1)).to_timestamp() as f64)
        .unwrap_or(0.0)
        .serialize(&SERIALIZER)
        .unwrap_throw()
}

#[wasm_bindgen]
pub fn wasm_pool_x_max(pool_data: *const Vec<Pool>) -> JsValue {
    let pool_data = unsafe { pool_data.as_ref().unwrap_throw() };
    pool_data
        .last()
        .map(|pool| (pool.date + Months::new(1)).to_timestamp() as f64)
        .unwrap_or(0.0)
        .serialize(&SERIALIZER)
        .unwrap_throw()
}
