use crate::utils::hex_to_rgb;

mod utils;

mod spools;

use colored::Colorize;
use reqwest::Error;
use spools::Spool;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let req_url = format!(
        "https://spools.lan.dingsi.net/api/v1/{endpoint}",
        endpoint = "spool"
    );
    //println!("{}", req_url);

    let response = reqwest::get(&req_url).await?;

    let spools: Vec<Spool> = response.json().await?;
    //println!("{:#?}", spools);
    for spool in spools.iter() {
        //println!("{:#?}", spool);
        if !spool.archived {
            let (r, g, b) = hex_to_rgb(&spool.filament.color_hex).unwrap();

            println!(
                "{:4} {:48} {:10} {:^8} {:>10.2} {:>10.2} {:12}",
                spool.id,
                spool.filament.name,
                spool.filament.material,
                spool.filament.color_hex.on_truecolor(r, g, b),
                spool.remaining_weight,
                spool.used_weight,
                spool.last_used,
            );
        }
    }

    return Ok(());
}
