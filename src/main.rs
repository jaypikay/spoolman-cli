mod helpers;
mod spoolman;

use clap::{arg, Command};
use tokio::runtime::Runtime;

use spoolman::spool;

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
        .subcommand(
            Command::new("measured")
                .about("Set measured weight for a spool")
                .arg(arg!(<SPOOLID> "Spool ID").value_parser(clap::value_parser!(u32)))
                .arg(
                    arg!(<WEIGHT> "Measured weight of spool in grams")
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
            helpers::print_spool_table_header();
            for spool in spools.iter() {
                if !spool.archived {
                    helpers::print_spool_table_row(spool);
                }
            }
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}

fn use_spool_material(spool_id: &u32, weight: &f32) {
    let result = Runtime::new()
        .unwrap()
        .block_on(async { spool::get_spool_by_id(spool_id).await });

    match result {
        Ok(checked_spool) => {
            if weight <= &checked_spool.remaining_weight {
                let result = Runtime::new()
                    .unwrap()
                    .block_on(async { spool::use_spool_by_grams(spool_id, weight).await });

                match result {
                    Ok(used_spool) => {
                        helpers::print_spool_table_header();
                        helpers::print_spool_table_row(&used_spool);

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

fn set_measured_weight(spool_id: &u32, weight: &f32) {
    let result = Runtime::new()
        .unwrap()
        .block_on(async { spool::set_spool_weight(spool_id, weight).await });

    match result {
        Ok(used_spool) => {
            helpers::print_spool_table_header();
            helpers::print_spool_table_row(&used_spool);

            if weight < &0.0 {
                println!("Inplausible measured spool weight!")
            }
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}

fn check_material_available(spool_id: &u32, weight: &f32) {
    let result = Runtime::new()
        .unwrap()
        .block_on(async { spool::get_spool_by_id(spool_id).await });

    match result {
        Ok(spool) => {
            helpers::print_spool_table_header();
            helpers::print_spool_table_row(&spool);

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
        Some(("measured", sub_matches)) => set_measured_weight(
            sub_matches.get_one::<u32>("SPOOLID").expect("required"),
            sub_matches.get_one::<f32>("WEIGHT").expect("required"),
        ),
        _ => unreachable!(),
    }
}
