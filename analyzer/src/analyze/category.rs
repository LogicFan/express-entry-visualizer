use super::calc::{CategoryPool, ScorePool};
use crate::{data::{CategoryCode, Invite, Pool}, utils::console_log};
use chrono::{Days, NaiveDate};
use std::collections::{HashMap, HashSet};

pub struct CategoryAnalyzer;

impl CategoryAnalyzer {
    pub fn of_category_years<'a>(invite_data: &'a [Invite]) -> HashMap<i32, &'a [Invite]> {
        let mut map = HashMap::new();

        let (invite_data, mut year) = 'ret: {
            for i in 0..invite_data.len() {
                if let Some(year) = invite_data[i].category.year {
                    break 'ret (&invite_data[i..], year);
                }
            }

            // no category invitation available
            map.insert(0, &invite_data[0..0]);
            return map;
        };

        let mut i = 0;
        for j in 0..invite_data.len() {
            let year_next = invite_data[j].category.year.unwrap_or(0);
            if year < year_next {
                map.insert(year, &invite_data[i..j]);
                i = j;
                year = year_next;
            }
        }

        if !invite_data.is_empty() {
            map.insert(year, &invite_data[i..invite_data.len()]);
        }

        // insert all category invitation to 0
        map.insert(0, invite_data);

        map
    }

    pub fn invite_per_category(
        pool_data: &[Pool],
        invite_data: &[Invite],
    ) -> (Vec<NaiveDate>, Vec<CategoryPool>, HashSet<CategoryCode>) {
        if pool_data.len() == 0 || invite_data.len() == 0 {
            return (Vec::new(), Vec::new(), HashSet::new());
        }

        let i_0 = invite_data.first().unwrap().date;
        let i_n = invite_data.last().unwrap().date + Days::new(1);

        let (mut labels, mut values) = {
            let capacity = invite_data.len();
            (Vec::with_capacity(capacity), Vec::with_capacity(capacity))
        };
        let mut categories = HashSet::new();

        let mut pools: Vec<_> = pool_data.iter().copied().rev().collect();
        let mut invites: Vec<_> = invite_data.iter().copied().rev().collect();

        let mut pool_to_invite = None;
        let mut value = CategoryPool::zero();

        // for General draw, if pool information can be found, the > 600 goes to Province, other goes to General
        // otherwise, put in their respected category
        let mut i = i_0;
        while i < i_n {
            console_log!("i: {}", i);
            let mut i_next = i_n;

            // calculate the pool-based increase
            while let Some(pool) = pools.last() {
                if pool.date <= i {
                    console_log!("source_pool: {:?}", pool);
                    pool_to_invite = ScorePool::from(*pool).into();
                    pools.pop();
                } else {
                    break;
                }
            }

            // calculate the invite-based increase
            while let Some(invite) = invites.last() {
                assert!(invite.date >= i);
                if invite.date != i {
                    // potential next i: next invite data date
                    i_next = std::cmp::min(i_next, invite.date);
                    break;
                }

                let invite_as_pool = if let Some(pool) = pool_to_invite {
                    console_log!("pool_to_invite: {:?}", pool);
                    let invite_as_pool = pool.invite(invite);
                    pool_to_invite = Some(pool - invite_as_pool); // remove already invited candidates from the pool to avoid duplicate counts.
                    Some(invite_as_pool)
                } else {
                    None
                };

                if invite.category.code == CategoryCode::General {
                    if let Some(invite_as_pool) = invite_as_pool {
                        console_log!("invite_as_pool: {:?}", invite_as_pool);
                        value[CategoryCode::Province] += invite_as_pool.pnp().total();
                        value[CategoryCode::General] += invite_as_pool.non_pnp().total();
                        categories.insert(CategoryCode::Province);
                        categories.insert(CategoryCode::General);
                    } else {
                        value[CategoryCode::General] += invite.size;
                        categories.insert(CategoryCode::General);
                    }
                } else {
                    value[invite.category.code] += invite.size;
                    categories.insert(invite.category.code);
                }

                console_log!("invite: {}, value: {:?}", invite.id, value);

                invites.pop();
            }

            labels.push(i);
            values.push(value);

            i = i_next;
        }

        (labels, values, categories)
    }
}

#[cfg(test)]
mod tests {
    use super::CategoryAnalyzer;
    use crate::data::wasm_invite_data;
    use crate::data::InviteId;

    #[tokio::test]
    async fn of_category_years() {
        let invite_data = unsafe { wasm_invite_data().await.as_ref().unwrap() };
        let map = CategoryAnalyzer::of_category_years(invite_data);

        assert_eq!(map[&0].first().unwrap().id, InviteId::parse("252"));
        assert_eq!(map[&0].last().unwrap().id, invite_data.last().unwrap().id);

        assert_eq!(map[&2023].first().unwrap().id, InviteId::parse("252"));
        assert_eq!(map[&2023].last().unwrap().id, InviteId::parse("286"));

        assert_eq!(map[&2024].first().unwrap().id, InviteId::parse("287"));
    }
}
