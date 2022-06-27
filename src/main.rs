
use dotenv::dotenv;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use std::collections::HashMap;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_key = env::var("API_KEY")?;

    let mut headers = HeaderMap::new();

    headers.insert(AUTHORIZATION, format!("Bearer {}",api_key).parse().unwrap());

    let client = reqwest::Client::new();
    let resp = client
        .get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}