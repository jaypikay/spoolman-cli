mod spoolman;
mod utils;

use clap::{arg, Command};
use colored::Colorize;
use tokio::runtime::Runtime;

use spoolman::spool;
use utils::hex_to_rgb;

fn cli() -> Command {
    Command::new("spool")
        .about("A Spoolman CLI helper")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("list").about("List available spools"))
        .subcommand(
            Command::new("check")
                .about("Check if a spool has enough available material")
                .arg(arg!(<SPOOLID> "Spool ID").value_parser(clap::value_parser!(u32)))
                .arg(
                    arg!(<WEIGHT> "Used weight by print in grams")
                        .value_parser(clap::value_parser!(f32)),
                ),
        )
        .subcommand(
            Command::new("use")
                .about("Reduce the used filament in grams from the spool")
                .arg(arg!(<SPOOLID> "Spool ID").value_parser(clap::value_parser!(u32)))
                .arg(
                    arg!(<WEIGHT> "Used weight by print in grams")
                        .value_parser(clap::value_parser!(f32)),
                ),
        )
}

fn display_spools() {
    let result = Runtime::new()
        .unwrap()
        .block_on(async { spool::get_spools().await });

    match result {
        Ok(spools) => {
            println!(
                "{:^4} {:^48} {:10} {:^8} {:>10} {:^10} {:12}",
                "ID", "Filament", "Material", "Color", "Available", "Used", "Last used"
            );
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

fn use_spool_material(spool_id: &u32, weight: &f32) {
    let result = Runtime::new()
        .unwrap()
        .block_on(async { spool::get_spool(spool_id).await });

    match result {
        Ok(checked_spool) => {
            if weight <= &checked_spool.remaining_weight {
                let result = Runtime::new()
                    .unwrap()
                    .block_on(async { spool::use_spool(spool_id, weight).await });

                match result {
                    Ok(used_spool) => {
                        let (r, g, b) = hex_to_rgb(&used_spool.filament.color_hex).unwrap();

                        println!(
                            "{:^4} {:^48} {:10} {:^8} {:>10} {:^10} {:12}",
                            "ID", "Filament", "Material", "Color", "Available", "Used", "Last used"
                        );
                        println!(
                            "{:4} {:48} {:10} {:^8} {:>10.2} {:>10.2} {:12}",
                            used_spool.id,
                            used_spool.filament.name,
                            used_spool.filament.material,
                            used_spool.filament.color_hex.on_truecolor(r, g, b),
                            used_spool.remaining_weight,
                            used_spool.used_weight,
                            used_spool.last_used,
                        );

                        if weight > &used_spool.remaining_weight {
                            println!("Not enough filament on spool!");
                        }
                    }
                    Err(err) => eprintln!("Error: {}", err),
                }
            } else {
                println!("Not enough filament on spool!");
            }
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}

fn check_material_available(spool_id: &u32, weight: &f32) {
    let result = Runtime::new()
        .unwrap()
        .block_on(async { spool::get_spool(spool_id).await });

    match result {
        Ok(spool) => {
            let (r, g, b) = hex_to_rgb(&spool.filament.color_hex).unwrap();

            println!(
                "{:^4} {:^48} {:10} {:^8} {:>10} {:^10} {:12}",
                "ID", "Filament", "Material", "Color", "Available", "Used", "Last used"
            );
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

            if weight > &spool.remaining_weight {
                println!("Not enough filament on spool!");
            }
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("list", _)) => display_spools(),
        Some(("check", sub_matches)) => check_material_available(
            sub_matches.get_one::<u32>("SPOOLID").expect("required"),
            sub_matches.get_one::<f32>("WEIGHT").expect("required"),
        ),
        Some(("use", sub_matches)) => use_spool_material(
            sub_matches.get_one::<u32>("SPOOLID").expect("required"),
            sub_matches.get_one::<f32>("WEIGHT").expect("required"),
        ),
        _ => unreachable!(),
    }
}
