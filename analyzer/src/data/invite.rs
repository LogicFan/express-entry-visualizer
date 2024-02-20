use chrono::NaiveDate;
use itertools::Itertools;

use super::{
    raw::{EeRounds123En, RawData},
    Category, Pathway,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Invite {
    pub id: i64,
    pub date: NaiveDate,
    pub category: Category,
    pub pathway: Pathway,
    pub size: i64,
    pub score: i64,
}

fn parse_i64(x: &str) -> i64 {
    match x {
        "91a" | "91b" => 91,
        _ => x.replace(",", "").parse().unwrap_or(0),
    }
}

fn parse_date(x: &str) -> NaiveDate {
    NaiveDate::parse_from_str(x, "%B %d, %Y").unwrap_or(NaiveDate::MIN)
}

impl Invite {
    pub fn parse(raw_data: &RawData) -> Self {
        Self {
            id: parse_i64(&raw_data.draw_number),
            date: parse_date(&raw_data.draw_date_full),
            category: Category::parse(&raw_data.draw_name),
            pathway: Pathway::parse(&raw_data.draw_text2),
            size: parse_i64(&raw_data.draw_size),
            score: parse_i64(&raw_data.draw_crs),
        }
    }

    pub fn parse_all(raw_data: &EeRounds123En) -> Vec<Self> {
        raw_data
            .rounds
            .iter()
            .map(|round| Self::parse(round))
            .filter(|invitation| invitation.is_valid())
            .sorted_by_key(|invitation| invitation.date)
            .collect()
    }

    pub fn is_valid(&self) -> bool {
        self.id != 0
            && self.date != NaiveDate::MIN
            && self.category != Category::Invalid
            && self.pathway.is_valid()
            && self.size != 0
            && self.score != 0
    }
}

#[cfg(test)]
mod tests {
    use super::super::raw::raw_data;
    use super::Invite;
    use tokio;

    #[tokio::test]
    async fn parse_data() {
        let x = raw_data().await;
        let i = Invite::parse_all(&x);

        assert!(i.iter().all(|x| x.is_valid()));

        println!("{:?}", i);
    }
}
