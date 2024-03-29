#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
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

impl Category {
    pub fn values() -> &'static [Category] {
        return &[
            Category::General,
            Category::Province,
            Category::Inland,
            Category::Oversea,
            Category::Stem,
            Category::Health,
            Category::French,
            Category::Trade,
            Category::Transport,
            Category::Agriculture,
        ];
    }

    pub fn parse(raw_data: &str) -> Self {
        if raw_data == "No Program Specified" || raw_data == "General" {
            Category::General
        } else if raw_data == "Provincial Nominee Program" {
            Category::Province
        } else if raw_data == "Canadian Experience Class" {
            Category::Inland
        } else if raw_data == "Federal Skilled Worker" {
            Category::Oversea
        } else if raw_data == "Federal Skilled Trades" || raw_data.starts_with("Trade occupations")
        {
            Category::Trade
        } else if raw_data.starts_with("STEM occupations") {
            Category::Stem
        } else if raw_data.starts_with("Healthcare occupations") {
            Category::Health
        } else if raw_data.starts_with("French language proficiency") {
            Category::French
        } else if raw_data.starts_with("Transport occupations") {
            Category::Transport
        } else if raw_data.starts_with("Agriculture and agri-food occupations") {
            Category::Agriculture
        } else {
            Category::Invalid
        }
    }

    pub fn as_str(&self) -> String {
        match self {
            Category::General => "General".into(),
            Category::Province => "Province".into(),
            Category::Inland => "Canadian Experience".into(),
            Category::Oversea => "Foreign Worker".into(),
            Category::Stem => "STEM".into(),
            Category::Health => "Health".into(),
            Category::French => "French".into(),
            Category::Trade => "Trade".into(),
            Category::Transport => "Transport".into(),
            Category::Agriculture => "Agriculture".into(),
            Category::Invalid => "Unknown".into(),
        }
    }

    pub fn as_color(&self) -> String {
        match self {
            Category::General => "#ECF0F1".into(),
            Category::Province => "#9B59B6".into(),
            Category::Inland => "#E74C3C".into(),
            Category::Oversea => "#C0392B".into(),
            Category::Stem => "#3498DB".into(),
            Category::Health => "#16A085".into(),
            Category::French => "#D35400".into(),
            Category::Trade => "#7F8C8D".into(),
            Category::Transport => "#F39C12".into(),
            Category::Agriculture => "#2ECC71".into(),
            Category::Invalid => "#000000".into(),
        }
    }
}
