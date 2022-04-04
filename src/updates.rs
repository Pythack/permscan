use reqwest::blocking::Client;
use std::error::Error;
use std::io::{self, stdout, Write};
use subprocess::Exec;

#[path = "./misc.rs"]
mod misc;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn check_for_newer_version(build: bool) -> Result<(), Box<dyn Error>> {
    println!("\x1b[94mCurrent version: {}\x1b[0m", VERSION);
    print!("Checking latest version on GitHub... ");
    let _flush = io::stdout().flush();

    let client = Client::new();
    let body = client
        .get("https://api.github.com/repos/Pythack/permscan/releases")
        .header("User-Agent", "permscan update checker 1.0")
        .send();
    match body {
        Ok(body) => {
            if let Ok(response) = body.text() {
                let json: serde_json::Value =
                    match serde_json::from_str(&response) {
                        Ok(value) => value,
                        _ => return Err("parsing".into()),
                    };
                let latest = json.as_array().unwrap();
                if !latest.is_empty() {
                    if misc::rem_first(
                        latest[0]["tag_name"].as_str().unwrap(),
                        "v",
                    ) != VERSION
                    {
                        println!("\r\x1b[93mNewer version available: {}! Visit this url: {}\x1b[0m", misc::rem_first(latest[0]["tag_name"].as_str().unwrap(), "v"), latest[0]["html_url"].as_str().unwrap());
                        print!("Do you want to update ? (y/*) ");
                        let _flush = stdout().flush();
                        let mut answer = String::new();
                        io::stdin().read_line(&mut answer)?;
                        if answer.to_lowercase().trim() == "y" {
                            if let Err(e) = update(build) {
                                eprintln!("\x1b[91m{}\x1b[0m", e);
                                return Err("version".into());
                            }
                        }
                    } else {
                        println!("\r\x1b[92mYou have the latest version! Thank you for using permscan!\x1b[0m");
                    }
                }
            }
        }

        _ => {
            eprintln!("\n\x1b[91mpermscan: update: failed to connect to the github api. are you connected to the internet ?\x1b[0m");
            return Err("connection".into());
        }
    }
    Ok(())
}

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
