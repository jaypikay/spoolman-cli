use crate::spoolman::{
    api,
    filament::Filament,
    utils::{default_empty_float, default_empty_string},
};
use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Spool {
    pub id: u32,
    pub registered: String,
    #[serde(default = "default_empty_string")]
    pub first_used: String,
    #[serde(default = "default_empty_string")]
    pub last_used: String,
    pub filament: Filament,
    #[serde(default = "default_empty_float")]
    pub price: f32,
    #[serde(default = "default_empty_float")]
    pub remaining_weight: f32,
    #[serde(default = "default_empty_float")]
    pub used_weight: f32,
    #[serde(default = "default_empty_float")]
    pub remaining_length: f32,
    #[serde(default = "default_empty_float")]
    pub used_length: f32,
    #[serde(default = "default_empty_string")]
    pub location: String,
    #[serde(default = "default_empty_string")]
    pub lot_nr: String,
    #[serde(default = "default_empty_string")]
    pub comment: String,
    pub archived: bool,
}

pub async fn get_spools() -> Result<Vec<Spool>, Error> {
    let response = api::get("spool").await?;

    let spools: Vec<Spool> = response.json().await?;

    return Ok(spools);
}
