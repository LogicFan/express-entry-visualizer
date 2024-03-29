use crate::chart::dataset::{BarDataset, ChartData, LineDataset, Tooltip};
use crate::chart::utils::{ToTimestamp, SERIALIZER};
use crate::data::{CategoryCode, Invite};
use chrono::{Datelike, Days, Months, NaiveDate, Weekday};
use itertools::Itertools;
use serde::Serialize;
use wasm_bindgen::{prelude::*, throw_str};

#[wasm_bindgen]
pub fn wasm_invite_score_data(invitation_data: *const Vec<Invite>) -> JsValue {
    let source = unsafe { invitation_data.as_ref().unwrap_throw() };
    let labels = source
        .iter()
        .map(|invitation| invitation.date.to_timestamp() as f64)
        .collect::<Vec<_>>();
    let datasets = CategoryCode::values()
        .iter()
        .map(|category| {
            let data = source
                .iter()
                .map(|invitation| {
                    if invitation.category.code == *category {
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
pub fn wasm_invite_size_data(invitation_data: *const Vec<Invite>, mode: String) -> JsValue {
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

    let fn_bar_date = if mode == "d" {
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
        .group_by(|invitation| fn_bar_date(invitation.date))
        .into_iter()
        .map(|(bar_date, _)| bar_date.to_timestamp() as f64)
        .collect::<Vec<_>>();
    let datasets = CategoryCode::values()
        .iter()
        .map(|category| {
            let data = source
                .iter()
                .group_by(|invitation| fn_bar_date(invitation.date))
                .into_iter()
                .map(|(_, invitations)| {
                    Some(
                        invitations
                            .into_iter()
                            .map(|invitation| {
                                if invitation.category.code == *category {
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
                data,
                background_color: category.as_color(),
                border_color: category.as_color(),
                stack: "0".into(),
            }
        })
        .collect::<Vec<_>>();

    let tooltip_title = source
        .iter()
        .group_by(|invitation| fn_bar_date(invitation.date))
        .into_iter()
        .map(|(bar_date, invitations)| {
            let id = match invitations.minmax() {
                itertools::MinMaxResult::NoElements => "123".into(),
                itertools::MinMaxResult::OneElement(x) => format!("{}", x.id),
                itertools::MinMaxResult::MinMax(x, y) => format!("{} - {}", x.id, y.id),
            };

            let date = if mode == "d" {
                format!("{}", bar_date.format("%Y-%m-%d"))
            } else {
                let bar_date2 = if mode == "w" {
                    bar_date + Days::new(7)
                } else {
                    bar_date + Months::new(1)
                };
                format!(
                    "{} - {}",
                    bar_date.format("%Y-%m-%d"),
                    bar_date2.format("%Y-%m-%d")
                )
            };

            format!("{}({})", date, id)
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
pub fn wasm_invite_x_min(invitation_data: *const Vec<Invite>) -> JsValue {
    let source = unsafe { invitation_data.as_ref().unwrap_throw() };
    source
        .first()
        .map(|invitation| (invitation.date - Months::new(1)).to_timestamp() as f64)
        .unwrap_or(0.0)
        .serialize(&SERIALIZER)
        .unwrap()
}

#[wasm_bindgen]
pub fn wasm_invite_x_max(invitation_data: *const Vec<Invite>) -> JsValue {
    let source = unsafe { invitation_data.as_ref().unwrap_throw() };
    source
        .last()
        .map(|invitation| (invitation.date + Months::new(1)).to_timestamp() as f64)
        .unwrap_or(0.0)
        .serialize(&SERIALIZER)
        .unwrap()
}
