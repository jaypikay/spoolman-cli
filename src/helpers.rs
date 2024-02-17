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
        "{:^4} {:^48} {:10} {:^8} {:>10} {:^10} {:12}",
        "ID", "Filament", "Material", "Color", "Available", "Used", "Last used"
    );
}

pub fn print_spool_table_row(spool: &spool::Spool) {
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
