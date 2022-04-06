use std::error::Error;
use std::process::Command;

#[allow(dead_code)]
// run ls to get files
pub fn run_ls(args: String, path: &str) -> Result<String, Box<dyn Error>> {
    let output = Command::new("ls").arg(args).arg(path).output();

    match output {
        Ok(content) => {
            let stdout = String::from_utf8(content.stdout);
            match stdout {
                Err(_e) => Ok(String::from("")),
                Ok(out) => Ok(out),
            }
        }
        Err(_e) => Err("".into()),
    }
}

// remove first character from string
pub fn rem_first(string: &str) -> String {
    let mut chars = string.chars();
    chars.next();
    return String::from(chars.as_str());
}
