use reqwest::blocking::Client;
use std::process::Command;

const VERSION: &str = "v2.2.1";

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

pub fn check_for_newer_version() {
    let client = Client::new();
    let body = client
        .get("https://api.github.com/repos/Pythack/permscan/releases")
        .header("User-Agent", "permscan update checker 1.0")
        .send();
    if let Ok(body) = body {
        if let Ok(response) = body.text() {
            let json: serde_json::Value =
                serde_json::from_str(&response).expect("Failed to parse");
            let latest = json.as_array().unwrap();
            if !latest.is_empty() {
                if latest[0]["tag_name"] != VERSION {
                    println!("\x1b[93mNewer version available: {}! Visit this url: {}\x1b[0m", rem_first(latest[0]["tag_name"].as_str().unwrap(), "v"), latest[0]["html_url"].as_str().unwrap());
                } else {
                    println!("\x1b[92mYou have the latest version!\x1b[0m");
                }
            }
        }
    }
}
