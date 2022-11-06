use reqwest::blocking::Client;
use std::io::{self, stdout, Write};
use subprocess::Exec;

#[path = "./misc.rs"]
mod misc;
#[path = "./types.rs"]
mod types;

use types::Result;

// Gets the current version from the Cargo.toml file.
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn check_for_newer_version(build: bool) -> Result<()> {
    info!("Current version: {}", VERSION);
    print!("Checking latest version on GitHub... ");
    let _flush = stdout().flush();

    let api_response = request_latest_version();

    match api_response {
        Ok(body) => {
            let response_str = response_to_str(body)?;
            let json = str_to_json(response_str)?;
            let latest = json_to_vec(json)?;
            if !latest.is_empty() {
                // compare latest release to current version
                if misc::remove_first_char(latest[0]["tag_name"].as_str().unwrap()) != VERSION {
                    info!(
                        "Newer version available: {}! Visit this url: {}",
                        misc::remove_first_char(latest[0]["tag_name"].as_str().unwrap()),
                        latest[0]["html_url"].as_str().unwrap(),
                    );
                    ask_for_update(build)?
                } else {
                    info!("You have the latest version! Thank you for using permscan!");
                }
            }
        }

        Err(e) => {
            return Err(format!(
                "updater: failed to connect to the github api: {}. are you connected to the internet ?",
                e
            )
            .into());
        }
    }
    Ok(())
}

fn request_latest_version() -> std::result::Result<reqwest::blocking::Response, reqwest::Error> {
    let client = Client::new();
    client
        .get("https://api.github.com/repos/Pythack/permscan/releases")
        .header("User-Agent", "permscan update checker 1.0")
        .send()
}

fn response_to_str(response: reqwest::blocking::Response) -> Result<String> {
    return match response.text() {
        Ok(str) => Ok(str),
        Err(e) => {
            return Err(format!("updater: failed to parse github api response: {}", e).into());
        }
    };
}

fn str_to_json(str: String) -> Result<serde_json::Value> {
    let json: serde_json::Value = match serde_json::from_str(&str) {
        Ok(json) => json,
        Err(e) => {
            return Err(format!("updater: failed to parse github api response: {}", e).into());
        }
    };
    Ok(json)
}

fn json_to_vec(json: serde_json::Value) -> Result<Vec<serde_json::Value>> {
    return match json.as_array() {
        Some(val) => Ok(val.to_vec()),
        None => Err("updater: failed to parse github api response".into()),
    };
}

fn ask_for_update(build: bool) -> Result<()> {
    print!("Do you want to update ? (y/*) ");
    let mut answer = String::new();
    get_input(&mut answer)?;
    if answer.to_lowercase().trim() == "y" {
        if let Err(e) = update(build) {
            return Err(e);
        }
    }
    Ok(())
}

fn get_input(buffer: &mut String) -> Result<&str> {
    let _flush = stdout().flush();
    match io::stdin().read_line(buffer) {
        Ok(_) => Ok(buffer),
        Err(e) => {
            error!("\nupdater: failed to read input: {}. please retry: \n", e,);
            get_input(buffer)
        }
    }
}

// Uses permscan-installer to install
// the newest version (overwrite the current version).
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
