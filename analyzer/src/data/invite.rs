use super::raw::{EeRounds123En, RawData};
use super::utils::{parse_date, parse_i32};
use super::{Category, Pathway};
use chrono::NaiveDate;
use itertools::Itertools;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct InviteId(i32, i32);

impl std::fmt::Display for InviteId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self(91, 0) => "91a".fmt(f),
            Self(91, 1) => "91b".fmt(f),
            Self(id, _) => id.fmt(f),
        }
    }
}

impl InviteId {
    fn parse(x: &str) -> Self {
        match x {
            "91a" => Self(91, 0),
            "91b" => Self(91, 1),
            _ => Self(parse_i32(x), 0),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Invite {
    pub id: InviteId,
    pub date: NaiveDate,
    pub category: Category,
    pub pathway: Pathway,
    pub size: f64,
    pub score: f64,
}

impl PartialEq for Invite {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Invite {}

impl PartialOrd for Invite {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Invite {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl Invite {
    pub fn parse(raw_data: &RawData) -> Self {
        Self {
            id: InviteId::parse(&raw_data.draw_number),
            date: parse_date(&raw_data.draw_date_full),
            category: Category::parse(&raw_data.draw_name),
            pathway: Pathway::parse(&raw_data.draw_text2),
            size: parse_i32(&raw_data.draw_size) as f64,
            score: parse_i32(&raw_data.draw_crs) as f64,
        }
    }

    // ensure sorted
    pub fn parse_all(raw_data: &EeRounds123En) -> Vec<Self> {
        raw_data
            .rounds
            .iter()
            .map(|round| Self::parse(round))
            .filter(|invitation| invitation.is_valid())
            .sorted()
            .collect::<Vec<_>>()
    }

    pub fn is_valid(&self) -> bool {
        self.category.is_valid() && self.pathway.is_valid()
    }
}

#[cfg(test)]
mod tests {
    use super::super::raw::raw_data;
    use super::{Invite, InviteId};
    use itertools::Itertools;

    #[tokio::test]
    async fn parse_data() {
        let x = raw_data().await;
        let i = Invite::parse_all(&x);

        assert!(i.iter().all(|x| x.is_valid()));
    }

    #[tokio::test]
    async fn invite_id() {
        let actual0 = vec!["1", "91b", "91a", "33", "100"]
            .into_iter()
            .map(InviteId::parse)
            .sorted()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>();
        let expect0 = vec!["1", "33", "91a", "91b", "100"]
            .into_iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>();

        assert_eq!(actual0, expect0);

        assert_eq!(format!("{}", InviteId::parse("33")), "33");
        assert_eq!(format!("{}", InviteId::parse("91a")), "91a");
        assert_eq!(format!("{}", InviteId::parse("91b")), "91b");
    }
}
