use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CategoryCode {
    General,
    Province,
    Inland,
    Oversea,
    // occupation categories
    Stem,
    Health,
    French,
    Trade,
    Transport,
    Agriculture,
    // unknown
    Invalid,
}

impl CategoryCode {
    pub fn values() -> &'static [CategoryCode] {
        return &[
            Self::General,
            Self::Province,
            Self::Inland,
            Self::Oversea,
            Self::Stem,
            Self::Health,
            Self::French,
            Self::Trade,
            Self::Transport,
            Self::Agriculture,
        ];
    }

    pub fn as_str(&self) -> String {
        match self {
            Self::General => "General".into(),
            Self::Province => "Province".into(),
            Self::Inland => "Canadian Experience".into(),
            Self::Oversea => "Foreign Worker".into(),
            Self::Stem => "STEM".into(),
            Self::Health => "Health".into(),
            Self::French => "French".into(),
            Self::Trade => "Trade".into(),
            Self::Transport => "Transport".into(),
            Self::Agriculture => "Agriculture".into(),
            Self::Invalid => "Unknown".into(),
        }
    }

    pub fn as_color(&self) -> String {
        match self {
            Self::General => "#ECF0F1".into(),
            Self::Province => "#9B59B6".into(),
            Self::Inland => "#E74C3C".into(),
            Self::Oversea => "#C0392B".into(),
            Self::Stem => "#3498DB".into(),
            Self::Health => "#16A085".into(),
            Self::French => "#D35400".into(),
            Self::Trade => "#7F8C8D".into(),
            Self::Transport => "#F39C12".into(),
            Self::Agriculture => "#2ECC71".into(),
            Self::Invalid => "#000000".into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Category {
    pub code: CategoryCode,
    pub year: Option<i32>,
}

impl Category {
    fn parse_year(raw_data: &str) -> Option<i32> {
        let re = Regex::new(r"\(20[0-9][0-9]-[0-9]\)").unwrap();
        
        re.find(raw_data).map(|m| {
            format!("{}", &m.as_str()[1..5]).parse().unwrap()
        })
    }

    pub fn parse(raw_data: &str) -> Self {
        let code = if raw_data == "No Program Specified" || raw_data == "General" {
            CategoryCode::General
        } else if raw_data == "Provincial Nominee Program" {
            CategoryCode::Province
        } else if raw_data == "Canadian Experience Class" {
            CategoryCode::Inland
        } else if raw_data == "Federal Skilled Worker" {
            CategoryCode::Oversea
        } else if raw_data == "Federal Skilled Trades" || raw_data.starts_with("Trade occupations")
        {
            CategoryCode::Trade
        } else if raw_data.starts_with("STEM occupations") {
            CategoryCode::Stem
        } else if raw_data.starts_with("Healthcare occupations") {
            CategoryCode::Health
        } else if raw_data.starts_with("French language proficiency") {
            CategoryCode::French
        } else if raw_data.starts_with("Transport occupations") {
            CategoryCode::Transport
        } else if raw_data.starts_with("Agriculture and agri-food occupations") {
            CategoryCode::Agriculture
        } else {
            CategoryCode::Invalid
        };

        Category {
            code,
            year: Category::parse_year(raw_data),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.code != CategoryCode::Invalid
    }

    pub fn as_str(&self) -> String {
        self.code.as_str()
    }

    pub fn as_color(&self) -> String {
        self.code.as_color()
    }
}
