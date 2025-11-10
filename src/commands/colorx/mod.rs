use crate::errors::Result;
use clap::Args;
use regex::Regex;

#[derive(Args)]
pub struct ColorXArgs {
    /// Color value(s) to convert.
    /// Accepts: hex (#fff or #ffffff), RGB components (255 255 255), or rgb() format (rgb(255, 255, 255))
    #[arg(required = true, num_args = 1..)]
    pub color: Vec<String>,
}

/// Returns true when the provided string is in a valid hex format.
fn is_hex_value(color: &str) -> bool {
    let regular_expression = Regex::new(r"(#[a-fA-F0-9]{3}|#[a-fA-F0-9]{6})$").unwrap();
    regular_expression.is_match(color)
}

/// Returns true when the provided string is in a valid RGB format.
fn is_rgb_value(color: &str) -> bool {
    // This is some gnarly regex, claude created it and so far so good.
    // It should match a wide range of rgb formats that I could paste in,
    // rgb(255,255,255), rgb(255 255 255), 255,255,255, 255 255 255, etc.
    let regular_expression = Regex::new(r"^(?:rgb\()? ?(25[0-5]|2[0-4][0-9]|[01]?[0-9]?[0-9])(?:, ?(25[0-5]|2[0-4][0-9]|[01]?[0-9]?[0-9])){2}\)?$|^(?:rgb\()? ?(25[0-5]|2[0-4][0-9]|[01]?[0-9]?[0-9])(?: (25[0-5]|2[0-4][0-9]|[01]?[0-9]?[0-9])){2}\)?$").unwrap();
    regular_expression.is_match(color)
}

/// Converts a hexadecimal color string to its RGB representation.
/// Assumes that the input string is a valid hex format.
fn convert_hex_to_rgb(hex: &str) -> String {
    let raw_hex = hex.trim_start_matches("#");

    let (r, g, b) = if raw_hex.len() == 3 {
        let chars: Vec<char> = raw_hex.chars().collect();
        (
            format!("{}{}", chars[0], chars[0]),
            format!("{}{}", chars[1], chars[1]),
            format!("{}{}", chars[2], chars[2]),
        )
    } else {
        let red = &raw_hex[..2];
        let green = &raw_hex[2..4];
        let blue = &raw_hex[4..6];

        (red.to_string(), green.to_string(), blue.to_string())
    };

    let red_base_10 = u8::from_str_radix(&r, 16).unwrap();
    let green_base_10 = u8::from_str_radix(&g, 16).unwrap();
    let blue_base_10 = u8::from_str_radix(&b, 16).unwrap();

    format!("rgb({}, {}, {})", red_base_10, green_base_10, blue_base_10)
}

/// Converts an RGB color string to its hexadecimal representation.
fn convert_rgb_to_hex(rgb: &str) -> String {
    let (r, g, b) = get_rgb_components(rgb);
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

/// Extracts the RGB components from a given RGB string.
/// Assumes that the input string is a valid RGB format.
fn get_rgb_components(rgb: &str) -> (u8, u8, u8) {
    let sanitized_rgb = rgb
        .replace("rgb", "")
        .replace("(", "")
        .replace(")", "")
        .replace(",", " ")
        .trim()
        .to_string();

    let parts = sanitized_rgb
        .split_whitespace()
        .filter_map(|s| s.parse::<u8>().ok())
        .collect::<Vec<u8>>();

    (parts[0], parts[1], parts[2])
}

pub fn run(args: &ColorXArgs) -> Result<()> {
    let color = args.color.join(" ");

    let converted_value = if is_hex_value(&color) {
        println!("Converting {} to RGB", &color);
        Some(convert_hex_to_rgb(&color))
    } else if is_rgb_value(&color) {
        println!("Converting {} to Hex", &color);
        Some(convert_rgb_to_hex(&color))
    } else {
        println!("Unrecognized color or format: {}", color);
        None
    };

    if let Some(value) = converted_value {
        println!("{}", value);
    }

    Ok(())
}
