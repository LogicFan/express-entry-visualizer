mod category;
mod invite;
mod pathway;
mod pool;
mod raw;

use async_once_cell::OnceCell;
use raw::raw_data;
use wasm_bindgen::prelude::wasm_bindgen;

pub use category::{Category, CategoryCode};
pub use invite::{Invite, InviteId};
pub use pathway::Pathway;
pub use pool::Pool;

#[wasm_bindgen]
pub async fn wasm_invite_data() -> *const Vec<Invite> {
    static DATA: OnceCell<Vec<Invite>> = OnceCell::new();
    return DATA
        .get_or_init(async { Invite::parse_all(raw_data().await) })
        .await;
}

#[wasm_bindgen]
pub async fn wasm_pool_data() -> *const Vec<Pool> {
    static DATA: OnceCell<Vec<Pool>> = OnceCell::new();
    return DATA
        .get_or_init(async { Pool::parse_all(raw_data().await) })
        .await;
}

mod utils {
    use chrono::NaiveDate;
    use wasm_bindgen::UnwrapThrowExt;

    pub fn parse_date(x: &str) -> NaiveDate {
        NaiveDate::parse_from_str(x, "%B %d, %Y").unwrap_throw()
    }

    pub fn parse_i32(x: &str) -> i32 {
        x.replace(",", "").parse().unwrap_throw()
    }
}
