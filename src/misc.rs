use std::path::Path;

#[path = "./colors.rs"]
mod colors;
#[path = "./types.rs"]
mod types;

use types::Result;

// remove first character from string
pub fn rem_first(string: &str) -> &str {
    let mut chars = string.chars();
    chars.next();
    chars.as_str()
}

// check if the path entered by the user exists and return
// an error if it doesn't
pub fn check_path_exists(path: &str) -> Result<()> {
    let path_exists = Path::new(&path).exists();
    if !path_exists {
        eprintln!(
            "{}permscan: {}: No such file or directory\x1b[0m{}",
            colors::RED,
            &path,
            colors::RESET
        );
        return Err("".into());
    }
    Ok(())
}

pub fn print_exit_info(exit_code: i32) {
    if exit_code != 0 {
        println!("{}permscan: process exited with exit code {}. to know more about error codes, visit https://github.com/Pythack/permscan/wiki/Error-codes{}", colors::RED, exit_code, colors::RESET)
    } else {
        eprintln!(
            "{}permscan: process successfully exited with exit code 0{}",
            colors::GREEN,
            colors::RESET
        )
    }
}

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
