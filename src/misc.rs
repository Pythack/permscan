//! miscellaneous functions


#[path = "./colors.rs"]
mod colors;
#[path = "./types.rs"]
mod types;

use types::Result;

// Removes first character from string
pub fn rem_first(string: &str) -> &str {
    let mut chars = string.chars();
    chars.next();
    chars.as_str()
}

// Verifies if any types arguments given by the user is valid
pub fn verify_type_argument(type_arg: &str) -> Result<()> {
    let possible_types = ["-", "d", "b", "c", "p", "l", "s"];

    if !possible_types.contains(&type_arg) {
        eprintln!(
            "{}permscan: {}: invalid type argument{}",
            colors::RED,
            type_arg,
            colors::RESET
        );
        return Err("typeArgErr".into());
    }
    Ok(())
}
