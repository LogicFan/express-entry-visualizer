extern crate wasm_bindgen;

use async_once_cell::OnceCell;
use serde::Deserialize;
use wasm_bindgen::prelude::*;

#[derive(Deserialize, Clone, Debug)]
pub struct EeRounds123En {
    pub classes: String,
    pub rounds: Vec<RawData>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct RawData {
    #[serde(rename = "DrawText1")]
    pub draw_text1: String,
    pub dd1: String,
    pub dd2: String,
    pub dd3: String,
    pub dd4: String,
    pub dd5: String,
    pub dd6: String,
    pub dd7: String,
    pub dd8: String,
    pub dd9: String,
    pub dd10: String,
    pub dd11: String,
    pub dd12: String,
    pub dd13: String,
    pub dd14: String,
    pub dd15: String,
    pub dd16: String,
    pub dd17: String,
    pub dd18: String,
    #[serde(rename = "drawCRS")]
    pub draw_crs: String,
    #[serde(rename = "drawCutOff")]
    pub draw_cutoff: String,
    #[serde(rename = "drawDate")]
    pub draw_date: String,
    #[serde(rename = "drawDateFull")]
    pub draw_date_full: String,
    #[serde(rename = "drawDateTime")]
    pub draw_date_time: String,
    #[serde(rename = "drawDistributionAsOn")]
    pub draw_distribution_as_on: String,
    #[serde(rename = "drawName")]
    pub draw_name: String,
    #[serde(rename = "drawNumber")]
    pub draw_number: String,
    #[serde(rename = "drawNumberURL")]
    pub draw_number_url: String,
    #[serde(rename = "drawSize")]
    pub draw_size: String,
    #[serde(rename = "drawText2")]
    pub draw_text2: String,
    pub mitext: String,
}

async fn impl_raw_data() -> EeRounds123En {
    reqwest::get("https://www.canada.ca/content/dam/ircc/documents/json/ee_rounds_123_en.json")
        .await
        .unwrap_throw()
        .json::<EeRounds123En>()
        .await
        .unwrap_throw()
}

pub async fn raw_data() -> &'static EeRounds123En {
    static DATA: OnceCell<EeRounds123En> = OnceCell::new();
    return DATA.get_or_init(impl_raw_data()).await;
}

#[cfg(test)]
mod tests {
    use super::raw_data;

    #[tokio::test]
    async fn parse_data() {
        let x = raw_data().await;
        println!("{:?}", x);
    }
}
