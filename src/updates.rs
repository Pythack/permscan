use reqwest::blocking::Client;
use std::error::Error;
use std::io::{self, stdout, Write};
use subprocess::Exec;

#[path = "./misc.rs"]
mod misc;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn check_for_newer_version() -> Result<(), Box<dyn Error>> {
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
                    serde_json::from_str(&response).expect("Failed to parse");
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
                        io::stdin()
                            .read_line(&mut answer)
                            .expect("Failed to read input");
                        if answer.to_lowercase().trim() == "y" {
                            let mut version = String::new();
                            print!("What version (linux-gnu, linux-musl, macos-arm, macos-x86_64) ");
                            let _flush = stdout().flush();
                            io::stdin()
                                .read_line(&mut version)
                                .expect("Failed to read input");

                            if let Err(e) =
                                update(version.to_lowercase().trim())
                            {
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

pub fn update(version: &str) -> Result<(), Box<dyn Error>> {
    match version {
        "linux-gnu" => {
            Exec::shell("wget https://github.com/Pythack/permscan/releases/latest/download/permscan-x86_64-unknown-linux-gnu.tar.gz").join()?;
            Exec::shell("tar -xzvf permscan-x86_64-unknown-linux-gnu.tar.gz")
                .join()?;
            Exec::shell(
                "sudo mv permscan-x86_64-unknown-linux-gnu/permscan /bin",
            )
            .join()?;
            Exec::shell("rm -rf permscan-x86_64-unknown-linux-gnu.tar.gz")
                .join()?;
            Exec::shell("rm -rf permscan-x86_64-unknown-linux-gnu").join()?;
            Ok(())
        }
        "linux-musl" => {
            Exec::shell("wget https://github.com/Pythack/permscan/releases/latest/download/permscan-x86_64-unknown-linux-musl.tar.gz").join()?;
            Exec::shell("tar -xzvf permscan-x86_64-unknown-linux-musl.tar.gz")
                .join()?;
            Exec::shell(
                "sudo mv permscan-x86_64-unknown-linux-musl/permscan /bin",
            )
            .join()?;
            Exec::shell("rm -rf permscan-x86_64-unknown-linux-musl.tar.gz")
                .join()?;
            Exec::shell("rm -rf permscan-x86_64-unknown-linux-musl").join()?;
            Ok(())
        }
        "macos-arm" => {
            Exec::shell("wget https://github.com/Pythack/permscan/releases/latest/download/permscan-aarch64-apple-darwin.zip").join()?;
            Exec::shell("unzip permscan-aarch64-apple-darwin.zip").join()?;
            Exec::shell(
                "sudo mv permscan-aarch64-apple-darwin/permscan /usr/local/bin",
            )
            .join()?;
            Exec::shell("rm -rf permscan-aarch64-apple-darwin.zip").join()?;
            Exec::shell("rm -rf permscan-aarch64-apple-darwin").join()?;
            Exec::shell("rm -rf __MACOSX").join()?;
            Ok(())
        }
        "macos-x86_64" => {
            Exec::shell("wget https://github.com/Pythack/permscan/releases/latest/download/permscan-x86_64-apple-darwin.zip").join()?;
            Exec::shell("unzip permscan-x86_64-apple-darwin.zip").join()?;
            Exec::shell(
                "sudo mv permscan-x86_64-apple-darwin/permscan /usr/local/bin",
            )
            .join()?;
            Exec::shell("rm -rf permscan-x86_64-apple-darwin.zip").join()?;
            Exec::shell("rm -rf permscan-x86_64-apple-darwin").join()?;
            Exec::shell("rm -rf __MACOSX").join()?;
            Ok(())
        }
        _ => {
            return Err(
                format!("permscan: {}: version not found", &version).into()
            )
        }
    }
}
