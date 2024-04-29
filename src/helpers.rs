use crate::spoolman::spool;
use colored::Colorize;

fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    if hex.len() != 6 {
        return None; // Invalid length for a hex color
    }

    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

    Some((r, g, b))
}

pub fn print_spool_table_header() {
    println!(
        "{:^4} {:48} {:9} {:^8} {:4} {:5} {:5} {:>9} {:>5} {:>7} {:20}",
        "ID".underline(),
        "Filament".underline(),
        "Material".underline(),
        "Color".underline(),
        "K".underline(),
        "Flow".underline(),
        "PA".underline(),
        "Remaining".underline(),
        "%".underline(),
        "Used".underline(),
        "Last used".underline()
    );
}

pub fn print_spool_table_row(spool: &spool::Spool) {
    let (r, g, b) = hex_to_rgb(&spool.filament.color_hex).unwrap();
    let pressure_advance = match spool.filament.extra.get("pressure_advance") {
        Some(value) => value,
        None => "",
    };
    let flow_rate = match spool.filament.extra.get("flow_rate") {
        Some(value) => value,
        None => "",
    };
    let k_factor = match spool.extra.get("k_factor") {
        Some(value) => value,
        None => "",
    };
    println!(
        "{:4} {:48} {:9} {:^8} {:4} {:5} {:5} {:>9.2} {:5.1} {:>7.2} {:20}",
        spool.id,
        spool.filament.name,
        spool.filament.material,
        spool.filament.color_hex.on_truecolor(r, g, b),
        k_factor,
        flow_rate,
        pressure_advance,
        spool.remaining_weight,
        spool.remaining_weight / spool.filament.weight * 100.,
        spool.used_weight,
        spool.last_used,
    );
}
