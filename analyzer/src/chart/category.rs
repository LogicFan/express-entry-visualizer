// use crate::analyze::category::CategoryAnalyzer;
// use crate::chart::dataset::{BarDataset, ChartData, LineDataset, Tooltip};
// use crate::chart::utils::{ToTimestamp, SERIALIZER};
// use crate::data::{Category, CategoryCode, Invite, Pool};
// use chrono::{Datelike, Days, Months, NaiveDate, Weekday};
// use itertools::Itertools;
// use serde::Serialize;
// use wasm_bindgen::{prelude::*, throw_str};

// #[wasm_bindgen]
// pub fn wasm_category_invite_data(
//     pool_data: *const Vec<Pool>,
//     invite_data: *const Vec<Invite>,
//     category_year: f64,
// ) -> JsValue {
//     let pool_data = unsafe { pool_data.as_ref().unwrap_throw() };
//     let invite_data = unsafe { invite_data.as_ref().unwrap_throw() };
//     let invite_data = CategoryAnalyzer::of_category_years(invite_data)[&(category_year as i32)];
//     let (category_invite_labels, category_invites) =
//         CategoryAnalyzer::invite_per_category(pool_data, invite_data);

//     let labels: Vec<_> = category_invite_labels
//         .iter()
//         .map(|date| date.to_timestamp() as f64)
//         .collect();

//     let datasets = CategoryCode::values()
//         .iter()
//         .map(|category| {
//             let data: Vec<_> = category_invites
//                 .iter()
//                 .map(|pool| pool.normalize())
//                 .map(|pool| match pool[*category] {
//                     0.0 => None,
//                     value => Some(value),
//                 })
//                 .collect::<Vec<_>>();

//             LineDataset {
//                 label: category.as_str(),
//                 data,
//                 background_color: category.as_color(),
//                 border_color: category.as_color(),
//                 ..Default::default()
//             }
//         })
//         .collect::<Vec<_>>();
//     let tooltip_title = source
//         .iter()
//         .map(|invitation| format!("{} ({})", invitation.date.format("%Y-%m-%d"), invitation.id))
//         .collect::<Vec<_>>();

//     ChartData {
//         labels,
//         datasets,
//         tooltip: Tooltip {
//             title: Some(tooltip_title),
//             label: None,
//         },
//     }
//     .serialize(&SERIALIZER)
//     .unwrap_throw()
// }
