use std::path::Path;

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
            "\x1b[91mpermscan: {}: No such file or directory\x1b[0m",
            &path
        );
        return Err("".into());
    }
    Ok(())
}

pub fn print_exit_info(exit_code: i32) {
    if exit_code != 0 {
        println!("\x1b[91mpermscan: process exited with exit code {}. to know more about error codes, visit https://github.com/Pythack/permscan/wiki/Error-codes\x1b[0m", exit_code)
    } else {
        eprintln!(
            "\x1b[92mpermscan: process successfully exited with exit code 0\x1b[0m"
        )
    }
}

pub fn verify_type_argument(type_arg: &str) -> Result<()> {
    let possible_types = ["-", "d", "b", "c", "p", "l", "s"];

    if !possible_types.contains(&type_arg) {
        eprintln!(
            "\x1b[91mpermscan: {}: invalid type argument\x1b[0m",
            type_arg
        );
        return Err("typeArgErr".into());
    }
    Ok(())
}
