use reqwest::blocking::Client;
use std::io::{self, stdout, Write};
use subprocess::Exec;

#[path = "./misc.rs"]
mod misc;
#[path = "./types.rs"]
mod types;

use types::Result;

// get the current version from cargo.toml
const VERSION: &str = env!("CARGO_PKG_VERSION");

// check for a newer version and if one exists, call ask_for_update()
pub fn check_for_newer_version(build: &bool) -> Result<()> {
    println!("\x1b[94mCurrent version: {}\x1b[0m", VERSION);
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
                    println!("\r\x1b[93mNewer version available: {}! Visit this url: {}\x1b[0m", misc::rem_first(latest[0]["tag_name"].as_str().unwrap()), latest[0]["html_url"].as_str().unwrap());
                    ask_for_update(build)?
                } else {
                    println!("\r\x1b[92mYou have the latest version! Thank you for using permscan!\x1b[0m");
                }
            }
        }

        Err(_) => {
            eprintln!("\n\x1b[91mpermscan: update: failed to connect to the github api. are you connected to the internet ?\x1b[0m");
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
            eprintln!("\n\x1b[91mpermscan: update: failed to parse github api response\x1b[0m");
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
            eprintln!("\n\x1b[91mpermscan: update: failed to parse github api response\x1b[0m");
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
            eprintln!("\n\x1b[91mpermscan: update: failed to parse github api response\x1b[0m");
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
            eprintln!("\x1b[91m{}\x1b[0m", e);
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
                "\n\x1b[91mpermscan: update: failed to read input. please retry: \x1b[0m\n"
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
