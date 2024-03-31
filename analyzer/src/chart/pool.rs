use super::dataset::PointStyle;
use super::utils::Stacker;
use crate::analyze::rate::RateAnalyzer;
use crate::analyze::smooth::Smoother;
use crate::chart::dataset::{ChartData, LineDataset, Tooltip};
use crate::chart::utils::{ToTimestamp, SERIALIZER};
use crate::data::{Invite, Pool};
use chrono::Days;
use serde::Serialize;
use std::iter;
use std::ops::Mul;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn wasm_pool_n() -> JsValue {
    Pool::N.serialize(&SERIALIZER).unwrap_throw()
}

#[wasm_bindgen]
pub fn wasm_pool_count_data(pool_data: *const Vec<Pool>) -> JsValue {
    let pool_data = unsafe { pool_data.as_ref().unwrap_throw() };
    let labels: Vec<_> = pool_data
        .iter()
        .map(|pool| pool.date.to_timestamp() as f64)
        .collect();
    let datasets: Vec<_> = (0..Pool::N)
        .rev()
        .into_iter()
        .map(|i| {
            let data: Vec<_> = pool_data
                .iter()
                .map(|pool| Some(Stacker::<{ Pool::N }, _>::new(*pool).rev(i)))
                .collect();

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
        .collect();

    ChartData {
        labels,
        datasets,
        tooltip: Tooltip::default(),
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

    let (rate_labels, mut rate_data) = RateAnalyzer::pool_increase_rate(pool_data, invite_data);
    let projected_rate = RateAnalyzer::projected_rate(&rate_data);
    Smoother::exponential(&rate_labels, &mut rate_data, 0.03278688524);

    let labels: Vec<_> = {
        assert!(!rate_labels.is_empty());
        let last_day = *rate_labels.last().unwrap();

        let extra_label = last_day + Days::new(120);
        rate_labels
            .iter()
            .chain([&extra_label])
            .map(|date| date.to_timestamp() as f64)
            .collect()
    };
    let actual = (0..Pool::N).into_iter().rev().map(|i| {
        let data: Vec<_> = rate_data
            .iter()
            .map(|rate| Some(Stacker::<{ Pool::N }, _>::new(*rate).rev(i)))
            .chain([None])
            .collect();

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
        let data: Vec<_> = iter::repeat(None)
            .take(rate_data.len() - 1)
            .chain([Some(
                Stacker::<{ Pool::N }, _>::new(*rate_data.last().unwrap()).rev(i),
            )])
            .chain([Some(Stacker::<{ Pool::N }, _>::new(projected_rate).rev(i))])
            .collect();

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

    let datasets: Vec<_> = predict.chain(actual).collect();

    ChartData {
        labels,
        datasets,
        tooltip: Tooltip {
            title: Vec::new(),
            label: (0..Pool::N)
                .rev()
                .map(|i| vec![format!("{}: {:.3}", Pool::as_str(i), projected_rate[i])])
                .collect(),
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
