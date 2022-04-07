use reqwest::blocking::Client;
use std::io::{self, stdout, Write};
use subprocess::Exec;

#[path = "./misc.rs"]
mod misc;
#[path = "./types.rs"]
mod types;

use types::Result;

use crate::colors;

// get the current version from cargo.toml
const VERSION: &str = env!("CARGO_PKG_VERSION");

// check for a newer version and if one exists, call ask_for_update()
pub fn check_for_newer_version(build: &bool) -> Result<()> {
    println!(
        "{}Current version: {}{}",
        colors::BLUE,
        VERSION,
        colors::RESET
    );
    print!("Checking latest version on GitHub... ");
    let _flush = io::stdout().flush();

    let api_response = request_latest_version();

    match api_response {
        Ok(body) => {
            let response_str = response_to_str(body)?;
            let json = str_to_json(response_str)?;
            let latest = json_to_vec(json)?;
            if !latest.is_empty() {
                // compare latest release to current version
                if misc::rem_first(latest[0]["tag_name"].as_str().unwrap())
                    != VERSION
                {
                    println!(
                        "\r{}Newer version available: {}! Visit this url: {}{}",
                        colors::YELLOW,
                        misc::rem_first(
                            latest[0]["tag_name"].as_str().unwrap()
                        ),
                        latest[0]["html_url"].as_str().unwrap(),
                        colors::RESET
                    );
                    ask_for_update(build)?
                } else {
                    println!("\r{}You have the latest version! Thank you for using permscan!{}", colors::GREEN, colors::RESET);
                }
            }
        }

        Err(_) => {
            eprintln!("\n{}permscan: update: failed to connect to the github api. are you connected to the internet ?{}", colors::RED, colors::RESET);
            return Err("connectionErr".into());
        }
    }
    Ok(())
}

fn request_latest_version(
) -> std::result::Result<reqwest::blocking::Response, reqwest::Error> {
    let client = Client::new();
    client
        .get("https://api.github.com/repos/Pythack/permscan/releases")
        .header("User-Agent", "permscan update checker 1.0")
        .send()
}

// wrapper around reqwest::blocking::Response.text() that allows
// to print a custom message when the Result is an error
fn response_to_str(response: reqwest::blocking::Response) -> Result<String> {
    return match response.text() {
        Ok(str) => Ok(str),
        Err(_) => {
            eprintln!(
                "\n{}permscan: update: failed to parse github api response{}",
                colors::RED,
                colors::RESET
            );
            return Err("parsingErr".into());
        }
    };
}

// wrapper around serde_json::from_str() that allows to print a custom message
// when the Result is an error
fn str_to_json(str: String) -> Result<serde_json::Value> {
    let json: serde_json::Value = match serde_json::from_str(&str) {
        Ok(json) => json,
        Err(_) => {
            eprintln!(
                "\n{}permscan: update: failed to parse github api response{}",
                colors::RED,
                colors::RESET
            );
            return Err("parsingErr".into());
        }
    };
    Ok(json)
}

// wrapper around json.as_array() that returns an error when the Option
// is None
fn json_to_vec(json: serde_json::Value) -> Result<Vec<serde_json::Value>> {
    return match json.as_array() {
        Some(val) => Ok(val.to_vec()),
        None => {
            eprintln!(
                "\n{}permscan: update: failed to parse github api response{}",
                colors::RED,
                colors::RESET
            );
            Err("parsingErr".into())
        }
    };
}

// ask the user if he wants to update
fn ask_for_update(build: &bool) -> Result<()> {
    print!("Do you want to update ? (y/*) ");
    let mut answer = String::new();
    get_input(&mut answer)?;
    if answer.to_lowercase().trim() == "y" {
        if let Err(e) = update(*build) {
            eprintln!("{}{}{}", colors::RED, e, colors::RESET);
            return Err("updateErr".into());
        }
    }
    Ok(())
}

// a wrapper around io::stdin().read_line() that retry when failing
fn get_input(buffer: &mut String) -> Result<&str> {
    let _flush = stdout().flush();
    match io::stdin().read_line(buffer) {
        Ok(_) => Ok(buffer),
        Err(_) => {
            eprintln!(
                "\n{}permscan: update: failed to read input. please retry: {}\n", colors::RED, colors::RESET
            );
            get_input(buffer)
        }
    }
}

// use permscan-installer to install the newest
// version (overwrite the current version)
fn update(build: bool) -> Result<()> {
    Exec::shell("wget https://raw.githubusercontent.com/Pythack/permscan/master/permscan-installer.sh").join()?;
    Exec::shell("chmod +x ./permscan-installer.sh").join()?;
    match build {
        true => {
            Exec::shell("./permscan-installer.sh -b").join()?;
        }
        false => {
            Exec::shell("./permscan-installer.sh").join()?;
        }
    }

    Ok(())
}
