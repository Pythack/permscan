use reqwest::blocking::Client;
use std::error::Error;
use std::io::{self, stdout, Write};
use subprocess::Exec;

#[path = "./misc.rs"]
mod misc;

// get the current version from cargo.toml
const VERSION: &str = env!("CARGO_PKG_VERSION");

// check for a newer version and if one exists, call ask_for_update()
pub fn check_for_newer_version(build: bool) -> Result<(), Box<dyn Error>> {
    println!("\x1b[94mCurrent version: {}\x1b[0m", VERSION);
    print!("Checking latest version on GitHub... ");
    let _flush = io::stdout().flush();

    // get the latest release from github api
    let client = Client::new();
    let body = client
        .get("https://api.github.com/repos/Pythack/permscan/releases")
        .header("User-Agent", "permscan update checker 1.0")
        .send();

    match body {
        Ok(body) => {
            if let Ok(response) = body.text() {
                let json: serde_json::Value = match serde_json::from_str(
                    &response,
                ) {
                    Ok(value) => value,
                    _ => {
                        eprintln!("\n\x1b[91mpermscan: update: failed to parse github api response\x1b[0m");
                        return Err("parsingErr".into());
                    }
                };
                let latest = json.as_array().unwrap();
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
        }

        _ => {
            eprintln!("\n\x1b[91mpermscan: update: failed to connect to the github api. are you connected to the internet ?\x1b[0m");
            return Err("connectionErr".into());
        }
    }
    Ok(())
}

// ask the user if he wants to update
pub fn ask_for_update(build: bool) -> Result<(), Box<dyn Error>> {
    print!("Do you want to update ? (y/*) ");
    let mut answer = String::new();
    get_input(&mut answer)?;
    if answer.to_lowercase().trim() == "y" {
        if let Err(e) = update(build) {
            eprintln!("\x1b[91m{}\x1b[0m", e);
            return Err("updateErr".into());
        }
    }
    Ok(())
}

// a wrapper around io::stdin().read_line() that retry when failing
fn get_input(buffer: &mut String) -> Result<&String, Box<dyn Error>> {
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
pub fn update(build: bool) -> Result<(), Box<dyn Error>> {
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
