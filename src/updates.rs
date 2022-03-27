use reqwest::blocking::Client;

const VERSION: &str = "v2.2.1";

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
                    println!("\x1b[93mNew version available! Visit this url: {}\x1b[0m", latest[0]["url"]);
                } else {
                    println!("You have the latest version!");
                }
            }
        }
    }
}
