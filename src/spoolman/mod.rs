mod api;
pub mod filament;
pub mod spool;
mod utils;
pub mod vendor;

//use colored::Colorize;
use reqwest::Error;

//use api::spoolman_get;
//use spool::Spool;
//use utils::hex_to_rgb;

pub async fn list_spools() -> Result<(), Error> {
    //let response = spoolman_get("").await?;
    //let spools: Vec<Spool> = response.json().await?;

    //for spool in spools.iter() {
    //    if !spool.archived {
    //        let (r, g, b) = hex_to_rgb(&spool.filament.color_hex).unwrap();

    //        println!(
    //            "{:4} {:48} {:10} {:^8} {:>10.2} {:>10.2} {:12}",
    //            spool.id,
    //            spool.filament.name,
    //            spool.filament.material,
    //            spool.filament.color_hex.on_truecolor(r, g, b),
    //            spool.remaining_weight,
    //            spool.used_weight,
    //            spool.last_used,
    //        );
    //    }
    //}

    return Ok(());
}
