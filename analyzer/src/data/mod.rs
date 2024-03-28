mod category;
mod invite;
mod pathway;
mod pool;
mod raw;

use async_once_cell::OnceCell;
use wasm_bindgen::prelude::wasm_bindgen;

use raw::raw_data;

pub use category::Category;
pub use invite::Invite;
pub use pathway::Pathway;
pub use pool::Pool;

pub enum PoolOrInvite {
    Pool(Pool),
    Invite(Invite)
}

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
