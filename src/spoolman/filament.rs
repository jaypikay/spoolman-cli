use crate::spoolman::{
    utils::{default_empty_float, default_empty_string, default_emtpy_u32},
    vendor::Vendor,
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Filament {
    pub id: u32,
    pub registered: String,
    pub name: String,
    pub vendor: Vendor,
    pub material: String,
    #[serde(default = "default_empty_float")]
    pub price: f32,
    #[serde(default = "default_empty_float")]
    pub density: f32,
    #[serde(default = "default_empty_float")]
    pub diameter: f32,
    #[serde(default = "default_empty_float")]
    pub weight: f32,
    #[serde(default = "default_empty_float")]
    pub spool_weight: f32,
    #[serde(default = "default_empty_string")]
    pub article_number: String,
    #[serde(default = "default_empty_string")]
    pub comment: String,
    #[serde(default = "default_emtpy_u32")]
    pub settings_extruder_temp: u32,
    #[serde(default = "default_emtpy_u32")]
    pub settings_bed_temp: u32,
    #[serde(default = "default_empty_string")]
    pub color_hex: String,
}
