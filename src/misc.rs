use std::process::Command;

pub fn run_command(command: String, args: String, path: String) -> String {
    let output = Command::new(command)
        .arg(args)
        .arg(path)
        .output()
        .expect("");
    let stdout = String::from_utf8(output.stdout);

    match stdout {
        Err(_e) => String::from(""),
        Ok(out) => out,
    }
}
pub fn rem_first(value: &str) -> String {
    let mut chars = value.chars();
    let first_value = match chars.next() {
        None => String::from(""),
        Some(value) => String::from(value),
    };
    if first_value == String::from('@') {
        return String::from(chars.as_str());
    } else {
        String::from(value)
    }
}
