use std::error::Error;
use std::process::Command;

#[allow(dead_code)] // For some reason, I get a dead code warning for
                    // run_command, despite it clearly being used.
pub fn run_command(
    command: String,
    args: String,
    path: String,
) -> Result<String, Box<dyn Error>> {
    let output = Command::new(command).arg(args).arg(path).output();

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
pub fn rem_first(value: &str, first_char: &str) -> String {
    let mut chars = value.chars();
    let first_value = match chars.next() {
        None => String::from(""),
        Some(value) => String::from(value),
    };
    if first_value == first_char {
        return String::from(chars.as_str());
    } else {
        String::from(value)
    }
}
