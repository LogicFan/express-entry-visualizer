mod category;
mod invite;
mod pathway;
mod pool;
mod raw;

use async_once_cell::OnceCell;
use itertools::Itertools;
use raw::raw_data;
use std::cmp::Ordering;
use wasm_bindgen::prelude::wasm_bindgen;

pub use category::{Category, CategoryCode};
pub use invite::{Invite, InviteId};
pub use pathway::Pathway;
pub use pool::Pool;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PoolOrInvite {
    Pool(Pool),
    Invite(Invite),
}

impl PartialOrd for PoolOrInvite {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PoolOrInvite {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (PoolOrInvite::Pool(lhs), PoolOrInvite::Pool(rhs)) => lhs.cmp(rhs),
            (PoolOrInvite::Invite(lhs), PoolOrInvite::Invite(rhs)) => lhs.cmp(rhs),
            (PoolOrInvite::Pool(lhs), PoolOrInvite::Invite(rhs)) => match lhs.date.cmp(&rhs.date) {
                Ordering::Equal => Ordering::Less,
                other => other,
            },
            (PoolOrInvite::Invite(lhs), PoolOrInvite::Pool(rhs)) => match lhs.date.cmp(&rhs.date) {
                Ordering::Equal => Ordering::Greater,
                other => other,
            },
        }
    }
}

impl PoolOrInvite {
    pub fn mix(
        invites: impl Iterator<Item = Invite>,
        pools: impl Iterator<Item = Pool>,
    ) -> Vec<Self> {
        let it_i = invites.map(|invite| PoolOrInvite::Invite(invite));
        let it_p = pools.map(|pool| PoolOrInvite::Pool(pool));
        it_i.chain(it_p).sorted().collect::<Vec<_>>()
    }
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
