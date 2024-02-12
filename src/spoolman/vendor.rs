use crate::spoolman::utils::default_empty_string;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Vendor {
    pub id: u32,
    pub registered: String,
    pub name: String,
    #[serde(default = "default_empty_string")]
    pub comment: String,
}
