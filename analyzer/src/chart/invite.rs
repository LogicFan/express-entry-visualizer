use crate::chart::dataset::{BarDataset, ChartData, LineDataset, Tooltip};
use crate::chart::utils::{ToTimestamp, SERIALIZER};
use crate::data::{CategoryCode, Invite};
use chrono::{Datelike, Days, Months, NaiveDate, Weekday};
use itertools::Itertools;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use wasm_bindgen::{prelude::*, throw_str};

#[wasm_bindgen]
pub fn wasm_invite_score_data(invite_data: *const Vec<Invite>) -> JsValue {
    let invite_data = unsafe { invite_data.as_ref().unwrap_throw() };
    let labels: Vec<_> = invite_data
        .iter()
        .map(|invitation| invitation.date.to_timestamp() as f64)
        .collect();
    let datasets: Vec<_> = CategoryCode::values()
        .iter()
        .map(|category| {
            let data: Vec<_> = invite_data
                .iter()
                .map(|invitation| {
                    if invitation.category.code == *category {
                        Some(invitation.score as f64)
                    } else {
                        None
                    }
                })
                .collect();

            LineDataset {
                label: category.as_str(),
                data,
                background_color: category.as_color(),
                border_color: category.as_color(),
                ..Default::default()
            }
        })
        .collect();
    let tooltip_title: Vec<_> = invite_data
        .iter()
        .map(|invitation| format!("{} ({})", invitation.date.format("%Y-%m-%d"), invitation.id))
        .collect();

    ChartData {
        labels,
        datasets,
        tooltip: Tooltip {
            title: vec![tooltip_title],
            label: Vec::new(),
        },
    }
    .serialize(&SERIALIZER)
    .unwrap_throw()
}

#[wasm_bindgen]
pub fn wasm_invite_size_data(invite_data: *const Vec<Invite>, mode: String) -> JsValue {
    static CACHE: Mutex<OnceLock<HashMap<String, ChartData<BarDataset>>>> =
        Mutex::new(OnceLock::new());
    {
        let mutex_guard = CACHE.lock().unwrap();
        let cache = mutex_guard.get_or_init(|| HashMap::new());
        match cache.get(&mode) {
            Some(value) => return value.serialize(&SERIALIZER).unwrap_throw(),
            None => (),
        }
    }

    let invite_data = unsafe { invite_data.as_ref().unwrap_throw() };
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

    let labels: Vec<_> = invite_data
        .iter()
        .group_by(|invitation| fn_bar_date(invitation.date))
        .into_iter()
        .map(|(bar_date, _)| bar_date.to_timestamp() as f64)
        .collect();
    let datasets: Vec<_> = CategoryCode::values()
        .iter()
        .map(|category| {
            let data: Vec<_> = invite_data
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
                .collect();

            BarDataset {
                label: category.as_str(),
                data,
                background_color: category.as_color(),
                border_color: category.as_color(),
                stack: "0".into(),
            }
        })
        .collect();

    let tooltip_title: Vec<_> = invite_data
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
        .collect();

    {
        let mut mutex_guard = CACHE.lock().unwrap();
        let cache = mutex_guard.get_mut().unwrap();
        cache.insert(
            mode.clone(),
            ChartData {
                labels,
                datasets,
                tooltip: Tooltip {
                    title: vec![tooltip_title],
                    label: Vec::new(),
                },
            },
        );
        cache[&mode].serialize(&SERIALIZER).unwrap_throw()
    }
}

#[wasm_bindgen]
pub fn wasm_invite_x_min(invite_data: *const Vec<Invite>) -> JsValue {
    let invite_data = unsafe { invite_data.as_ref().unwrap_throw() };
    invite_data
        .first()
        .map(|invitation| (invitation.date - Months::new(1)).to_timestamp() as f64)
        .unwrap_or(0.0)
        .serialize(&SERIALIZER)
        .unwrap()
}

#[wasm_bindgen]
pub fn wasm_invite_x_max(invite_data: *const Vec<Invite>) -> JsValue {
    let invite_data = unsafe { invite_data.as_ref().unwrap_throw() };
    invite_data
        .last()
        .map(|invitation| (invitation.date + Months::new(1)).to_timestamp() as f64)
        .unwrap_or(0.0)
        .serialize(&SERIALIZER)
        .unwrap()
}
