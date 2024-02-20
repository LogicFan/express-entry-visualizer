pub(crate) mod pool;

use wasm_bindgen::UnwrapThrowExt;

use crate::data::{self, Invite};

pub(crate) struct Analyzer;

// impl Analyzer {
//     pub fn pool_increase_rate(pool: &Vec<data::Pool>, invitation: &Vec<Invite>) -> Vec<data::Pool> {
//         if pool.len() == 0 {
//             return Vec::new();
//         }

//         let first_pool_date = pool.first().unwrap().date;
//         let last_pool_date = pool.last().unwrap().date;
//         let n = last_pool_date.signed_duration_since(first_pool_date);
//     }
// }