use clap::App;
use error_chain::error_chain;
use serde::Deserialize;

error_chain! {
    foreign_links {
        ReqwestError(reqwest::Error);
        ParseError(url::ParseError);
    }
}

#[tokio::main]
async fn main() {
    App::new("Get Public IP Address")
        .version("0.1.0")
        .get_matches();

    match get_ip().await {
        Ok(ip) => println!("Your Public IP is: {}", ip.ip),
        Err(err) => eprintln!("{}", err),
    }
}

#[derive(Deserialize, Debug)]
pub struct Ip {
    pub ip: String,
}

async fn get_ip() -> Result<Ip> {
    let url = url::Url::parse("https://api.ipify.org?format=json")?;
    let client = reqwest::Client::new();
    let ip: Ip = client
        .get(url)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .json()
        .await?;

    Ok(ip)
}
