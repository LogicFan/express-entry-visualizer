pub(crate) mod pool;

use crate::data;
use sorted_vec::SortedVec;

pub(crate) struct Analyzer;

impl Analyzer {
    pub fn pool_increase_rate(
        pool: &SortedVec<data::Pool>,
        invitation: &SortedVec<data::Invite>,
    ) -> Vec<data::Pool> {
        if pool.len() == 0 {
            return Vec::new();
        }
        todo!();
    }
}
