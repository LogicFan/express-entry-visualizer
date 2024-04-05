use super::dataset::{Dropdown, PointStyle};
use super::utils::Stacker;
use crate::analyze::category::CategoryAnalyzer;
use crate::chart::dataset::{ChartData, LineDataset, Tooltip};
use crate::chart::utils::{ToTimestamp, SERIALIZER};
use crate::data::{CategoryCode, Invite, Pool};
use itertools::Itertools;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn wasm_category_years(invite_data: *const Vec<Invite>) -> JsValue {
    let invite_data = unsafe { invite_data.as_ref().unwrap_throw() };
    CategoryAnalyzer::of_category_years(invite_data)
        .keys()
        .sorted()
        .map(|year| Dropdown {
            key: *year as f64,
            label: if *year == 0 {
                "all".into()
            } else {
                year.to_string()
            },
        })
        .collect::<Vec<_>>()
        .serialize(&SERIALIZER)
        .unwrap_throw()
}

#[wasm_bindgen]
pub fn wasm_category_invite_data(
    pool_data: *const Vec<Pool>,
    invite_data: *const Vec<Invite>,
    category_year: f64,
    with_pnp: bool,
) -> JsValue {
    let pool_data = unsafe { pool_data.as_ref().unwrap_throw() };
    let invite_data = unsafe { invite_data.as_ref().unwrap_throw() };
    let invite_data = CategoryAnalyzer::of_category_years(invite_data)[&(category_year as i32)];
    let (category_invite_labels, mut category_invites, mut categories) =
        CategoryAnalyzer::invite_per_category(pool_data, invite_data);

    if !with_pnp {
        category_invites
            .iter_mut()
            .for_each(|pool| pool[CategoryCode::Province] = 0.0);
        categories.remove(&CategoryCode::Province);
    }

    let labels: Vec<_> = category_invite_labels
        .iter()
        .map(|date| date.to_timestamp() as f64)
        .collect();

    let datasets = categories
        .iter()
        .sorted_by_key(|category| **category as usize)
        .map(|category| {
            let data: Vec<_> = category_invites
                .iter()
                .map(|pool| pool.normalize() * 100.0)
                .map(|pool| {
                    Some(Stacker::<{ CategoryCode::N }, _>::new(pool).val(*category as usize))
                })
                .collect();

            LineDataset {
                label: category.as_str(),
                data,
                background_color: category.as_color(),
                border_color: category.as_color(),
                fill: true,
                point_style: PointStyle(None),
                ..Default::default()
            }
        })
        .collect();

    let tooltip_title: Vec<_> = category_invite_labels
        .iter()
        .map(|date| format!("{}", date.format("%Y-%m-%d")))
        .collect();

    let tooltip_label: Vec<_> = categories
        .iter()
        .sorted_by_key(|category| **category as usize)
        .map(|category| {
            category_invites
                .iter()
                .map(|pool| {
                    format!(
                        "{}: {:.2}% ({})",
                        category.as_str(),
                        pool.normalize()[*category] * 100.0,
                        pool[*category] as i64
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect();

    ChartData {
        labels,
        datasets,
        tooltip: Tooltip {
            title: vec![tooltip_title],
            label: tooltip_label,
        },
    }
    .serialize(&SERIALIZER)
    .unwrap_throw()
}

#[wasm_bindgen]
pub fn wasm_category_pool_data(
    pool_data: *const Vec<Pool>,
    invite_data: *const Vec<Invite>,
    category_year: f64,
) -> JsValue {
    let pool_data = unsafe { pool_data.as_ref().unwrap_throw() };
    let invite_data = unsafe { invite_data.as_ref().unwrap_throw() };
    let invite_data = CategoryAnalyzer::of_category_years(invite_data)[&(category_year as i32)];
    let (category_invite_labels, category_invites, categories) =
        CategoryAnalyzer::percent_per_category(pool_data, invite_data);

    let labels: Vec<_> = category_invite_labels
        .iter()
        .map(|date| date.to_timestamp() as f64)
        .collect();

    let datasets = categories
        .iter()
        .sorted_by_key(|category| **category as usize)
        .map(|category| {
            let data: Vec<_> = category_invites
                .iter()
                .map(|pool| {
                    if pool[*category] == 0.0 {
                        None
                    } else {
                        Some(pool[*category] * 100.0)
                    }
                })
                .collect();

            LineDataset {
                label: category.as_str(),
                data,
                background_color: category.as_color(),
                border_color: category.as_color(),
                point_style: PointStyle(None),
                ..Default::default()
            }
        })
        .collect();

    let tooltip_title: Vec<_> = category_invite_labels
        .iter()
        .map(|date| format!("{}", date.format("%Y-%m-%d")))
        .collect();

    let tooltip_label: Vec<_> = categories
        .iter()
        .sorted_by_key(|category| **category as usize)
        .map(|category| {
            category_invites
                .iter()
                .map(|pool| format!("{}: {:.2}%", category.as_str(), pool[*category] * 100.0))
                .collect::<Vec<_>>()
        })
        .collect();

    ChartData {
        labels,
        datasets,
        tooltip: Tooltip {
            title: vec![tooltip_title],
            label: tooltip_label,
        },
    }
    .serialize(&SERIALIZER)
    .unwrap_throw()
}
