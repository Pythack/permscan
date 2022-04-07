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
