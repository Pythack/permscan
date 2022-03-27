use reqwest::{blocking::Client, header::USER_AGENT};

pub fn check_for_newer_version() {
    let client = Client::new();
    let body = client
        .get("https://api.github.com/repos/Pythack/permscan/releases/latest") //"https://api.github.com/repos/Pythack/permscan/tags"
        .header(USER_AGENT, "permscan update checker 1.0")
        .send();
    if let Ok(body) = body {
        println!("{:?}", body.text());
    } else {
        println!("Ok");
    }
}
