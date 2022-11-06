#[path = "./types.rs"]
mod types;

use std::path::Path;
use types::Result;

pub fn remove_first_char(string: &str) -> &str {
    let mut chars = string.chars();
    chars.next();
    chars.as_str()
}

pub fn verify_type_argument(type_arg: &str) -> Result<()> {
    let possible_types = ["-", "d", "b", "c", "p", "l", "s"];

    if !possible_types.contains(&type_arg) {
        return Err(format!("`{}`: invalid type argument", type_arg).into());
    }
    Ok(())
}

pub fn check_path_exists(path: &str) -> Result<()> {
    let path_exists = Path::new(&path).exists();
    if !path_exists {
        return Err(format!("No such file or directory: `{}`", path).into());
    }
    Ok(())
}
