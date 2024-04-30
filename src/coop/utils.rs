use colored::*;
pub fn colorize_str(color: usize, __str: String) -> colored::ColoredString {
    match color {
        0 => __str.purple(),
        1 => __str.red(),
        2 => __str.blue(),
        3 => __str.green(),
        4 => __str.purple(),
        _ => __str.white()
    }
}

pub fn colorize_bg_str(color: usize, __str: String) -> colored::ColoredString {
    match color {
        0 => __str.on_purple(),
        1 => __str.on_red(),
        2 => __str.on_blue(),
        3 => __str.on_green(),
        4 => __str.on_purple(),
        _ => __str.on_white()
    }
}