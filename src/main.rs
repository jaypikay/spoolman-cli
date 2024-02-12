mod spoolman;
mod utils;

use colored::Colorize;

use tokio::runtime::Runtime;

use spoolman::spool;
use utils::hex_to_rgb;

fn display_spools() {
    let result = Runtime::new()
        .unwrap()
        .block_on(async { spool::get_spools().await });

    match result {
        Ok(spools) => {
            for spool in spools.iter() {
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
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}

fn main() {
    display_spools();
}
