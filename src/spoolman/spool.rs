use reqwest::Error;
use serde::Deserialize;

use crate::spoolman::{
    api::SpoolmanAPI,
    filament::Filament,
    utils::{default_empty_float, default_empty_string},
};

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
    let api = SpoolmanAPI::new();
    let response = api
        .get("spool?sort=last_used:desc,filament.name:asc")
        .await?;

    let spools: Vec<Spool> = response.json().await?;

    Ok(spools)
}

pub async fn get_spool(spool_id: &u32) -> Result<Spool, Error> {
    let path = format!("spool/{}", spool_id);

    let api = SpoolmanAPI::new();
    let response = api.get(&path).await?;

    let spool: Spool = response.json().await?;

    Ok(spool)
}

pub async fn use_spool(spool_id: &u32, used_weight: &f32) -> Result<Spool, Error> {
    let path = format!("spool/{}/use", spool_id);

    let api = SpoolmanAPI::new();
    let params = vec![("use_weight", used_weight)];
    let response = api.put(&path, &params).await?;
    let spool: Spool = response.json().await?;

    Ok(spool)
}
