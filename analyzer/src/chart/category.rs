use super::utils::Stacker;
use crate::analyze::category::CategoryAnalyzer;
use crate::chart::dataset::{ChartData, LineDataset, Tooltip};
use crate::chart::utils::{ToTimestamp, SERIALIZER};
use crate::data::{CategoryCode, Invite, Pool};
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn wasm_category_years(
    invite_data: *const Vec<Invite>,
) -> JsValue {
    let invite_data = unsafe { invite_data.as_ref().unwrap_throw() };
    CategoryAnalyzer::of_category_years(invite_data)
        .keys()
        .filter(|year| **year != 0)
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
        .map(|category| {
            let data: Vec<_> = category_invites
                .iter()
                .map(|pool| pool.normalize() * 100.0)
                .map(|pool| {
                    if pool[*category] == 0.0 {
                        None
                    } else {
                        Some(Stacker::<{ CategoryCode::N }, _>::new(pool).val(*category as usize))
                    }
                })
                .collect::<Vec<_>>();

            LineDataset {
                label: category.as_str(),
                data,
                background_color: category.as_color(),
                border_color: category.as_color(),
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
