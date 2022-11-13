// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let efficiency = match speed {
        0 => 0.0,
        1..=4 => 1.0,
        5..=8 => 0.9,
        9.. => 0.77,
    };

    speed as f64 * 221.0 * efficiency
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let per_hour = production_rate_per_hour(speed);
    let per_minute = per_hour / 60.0;
    per_minute.floor() as u32
}
