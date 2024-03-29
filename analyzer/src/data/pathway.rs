#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pathway(u32);

impl Pathway {
    const PNP: u32 = 0x0001;
    const CEC: u32 = 0x0010;
    const FSW: u32 = 0x0100;
    const FST: u32 = 0x1000;

    pub fn is_pnp(&self) -> bool {
        (self.0 & Pathway::PNP) != 0
    }

    pub fn is_cec(&self) -> bool {
        (self.0 & Pathway::CEC) != 0
    }

    pub fn is_fsw(&self) -> bool {
        (self.0 & Pathway::FSW) != 0
    }

    pub fn is_fst(&self) -> bool {
        (self.0 & Pathway::FST) != 0
    }

    pub fn parse(raw_data: &str) -> Self {
        let mut x = 0;
        if raw_data.contains("Federal Skilled Worker") {
            x |= Pathway::FSW;
        }
        if raw_data.contains("Canadian Experience Class") {
            x |= Pathway::CEC;
        }
        if raw_data.contains("Federal Skilled Trades") {
            x |= Pathway::FST;
        }
        if raw_data.contains("Provincial Nominee Program") {
            x |= Pathway::PNP;
        }
        Self(x)
    }

    pub fn is_valid(&self) -> bool {
        self.is_cec() || self.is_fst() || self.is_fsw() || self.is_pnp()
    }
}
