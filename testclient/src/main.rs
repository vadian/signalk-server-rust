use std::collections::HashMap;
use reqwest::Response;
use reqwest::get;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = get("http://localhost:3000/heartbeat")
        .await?;

    println!("{:#?}", &resp);
    let text = &resp.text().await?;

    println!("{:#?}", text);
    Ok(())
}