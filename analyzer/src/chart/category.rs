// use crate::chart::dataset::{BarDataset, ChartData, LineDataset, Tooltip};
// use crate::chart::utils::{ToTimestamp, SERIALIZER};
// use crate::data::{Category, Invite};
// use chrono::{Datelike, Days, Months, NaiveDate, Weekday};
// use itertools::Itertools;
// use serde::Serialize;
// use wasm_bindgen::{prelude::*, throw_str};

// #[wasm_bindgen]
// pub fn wasm_category_invite_data(invitation_data: *const Vec<Invite>) -> JsValue {
//     let source = unsafe { invitation_data.as_ref().unwrap_throw() };
//     let labels = source
//         .iter()
//         .map(|invitation| invitation.date.to_timestamp() as f64)
//         .collect::<Vec<_>>();
//     let datasets = Category::values()
//         .iter()
//         .map(|category| {
//             let data = source
//                 .iter()
//                 .map(|invitation| {
//                     if invitation.category == *category {
//                         Some(invitation.score as f64)
//                     } else {
//                         None
//                     }
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
