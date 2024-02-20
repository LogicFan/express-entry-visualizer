use chrono::{Datelike, Days, Months, NaiveDate, Weekday};
use itertools::Itertools;
use serde::Serialize;
use wasm_bindgen::{prelude::*, throw_str};

use crate::chart::dataset::{BarDataset, ChartData, LineDataset, Tooltip};
use crate::chart::utils::{ToTimestamp, SERIALIZER};
use crate::data::{Category, Invite};

#[wasm_bindgen]
pub fn wasm_invitation_score_data(invitation_data: *const Vec<Invite>) -> JsValue {
    let source = unsafe { invitation_data.as_ref().unwrap_throw() };
    let labels = source
        .iter()
        .map(|invitation| invitation.date.to_timestamp() as f64)
        .collect::<Vec<_>>();
    let datasets = Category::values()
        .iter()
        .map(|category| {
            let data = source
                .iter()
                .map(|invitation| {
                    if invitation.category == *category {
                        Some(invitation.score as f64)
                    } else {
                        None
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
    let tooltip_title = source
        .iter()
        .map(|invitation| format!("{} ({})", invitation.date.format("%Y-%m-%d"), invitation.id))
        .collect::<Vec<_>>();

    ChartData {
        labels,
        datasets,
        tooltip: Tooltip {
            title: Some(tooltip_title),
            label: None,
        },
    }
    .serialize(&SERIALIZER)
    .unwrap_throw()
}

#[wasm_bindgen]
pub fn wasm_invitation_size_data(invitation_data: *const Vec<Invite>, mode: String) -> JsValue {
    let source = unsafe { invitation_data.as_ref().unwrap_throw() };
    fn per_day(date: NaiveDate) -> NaiveDate {
        date
    }
    fn per_week(date: NaiveDate) -> NaiveDate {
        date.week(Weekday::Mon).first_day()
    }
    fn per_month(date: NaiveDate) -> NaiveDate {
        NaiveDate::from_ymd_opt(date.year(), date.month(), 1).unwrap()
    }

    let bar_size = if mode == "d" {
        per_day
    } else if mode == "w" {
        per_week
    } else if mode == "m" {
        per_month
    } else {
        throw_str(format!("invalid mode {}", mode).as_str())
    };

    let labels = source
        .iter()
        .group_by(|invitation| bar_size(invitation.date))
        .into_iter()
        .map(|(bar_date, _invitations)| bar_date.to_timestamp() as f64)
        .collect::<Vec<_>>();
    let datasets = Category::values()
        .iter()
        .map(|category| {
            let data = source
                .iter()
                .group_by(|invitation| bar_size(invitation.date))
                .into_iter()
                .map(|(_bar_date, invitations)| {
                    Some(
                        invitations
                            .into_iter()
                            .map(|invitation| {
                                if invitation.category == *category {
                                    invitation.size as f64
                                } else {
                                    0_f64
                                }
                            })
                            .reduce(|x, y| x + y)
                            .unwrap_or(0_f64),
                    )
                })
                .collect::<Vec<_>>();

            BarDataset {
                label: category.as_str(),
                data: data,
                background_color: category.as_color(),
                border_color: category.as_color(),
                stack: "0".into(),
            }
        })
        .collect::<Vec<_>>();

    let tooltip_title = source
        .iter()
        .group_by(|invitation| bar_size(invitation.date))
        .into_iter()
        .map(|(bar_date, invitations)| {
            let mut min_id = i64::MAX;
            let mut max_id = i64::MIN;

            for invitation in invitations {
                if invitation.id < min_id {
                    min_id = invitation.id
                }
                if invitation.id > max_id {
                    max_id = invitation.id
                }
            }

            let min_date = bar_date;
            let max_date = if mode == "d" {
                bar_date
            } else if mode == "w" {
                bar_date.checked_add_days(Days::new(7)).unwrap_throw()
            } else {
                bar_date.checked_add_months(Months::new(1)).unwrap_throw()
            };

            fn format_collapse(from: String, to: String) -> String {
                if from == to {
                    return format!("{}", from);
                } else {
                    format!("{} to {}", from, to)
                }
            }

            format!(
                "{}({})",
                format_collapse(
                    min_date.format("%Y-%m-%d").to_string(),
                    max_date.format("%Y-%m-%d").to_string()
                ),
                format_collapse(min_id.to_string(), max_id.to_string())
            )
        })
        .collect::<Vec<_>>();

    ChartData {
        labels,
        datasets,
        tooltip: Tooltip {
            title: Some(tooltip_title),
            label: None,
        },
    }
    .serialize(&SERIALIZER)
    .unwrap_throw()
}

#[wasm_bindgen]
pub fn wasm_invitation_x_min(invitation_data: *const Vec<Invite>) -> JsValue {
    let source = unsafe { invitation_data.as_ref().unwrap_throw() };
    source
        .iter()
        .min_by_key(|invitation| invitation.date)
        .map(|invitation| {
            invitation
                .date
                .checked_sub_months(Months::new(1))
                .unwrap_throw()
                .to_timestamp() as f64
        })
        .unwrap_throw()
        .serialize(&SERIALIZER)
        .unwrap_throw()
}

#[wasm_bindgen]
pub fn wasm_invitation_x_max(invitation_data: *const Vec<Invite>) -> JsValue {
    let source = unsafe { invitation_data.as_ref().unwrap_throw() };
    source
        .iter()
        .max_by_key(|invitation| invitation.date)
        .map(|invitation| {
            invitation
                .date
                .checked_add_months(Months::new(1))
                .unwrap_throw()
                .to_timestamp() as f64
        })
        .unwrap_throw()
        .serialize(&SERIALIZER)
        .unwrap_throw()
}
