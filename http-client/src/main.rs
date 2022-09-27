use reqwest::get;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    //let result = heartbeat().await;
    let result = update_template().await;

    return match result {
        Ok(resp) => {
            tracing::debug!("Good result: {}", resp.status());
            let text = resp.text().await?;
            let pretty = serde_json::to_string_pretty(&text).unwrap();
            print!("{}", text);
            print!("{}", pretty);
            tracing::debug!("Contents: {}", text);
            tracing::debug!("Contents: {}", pretty);
            Ok(())
        },
        Err(e) => {
            tracing::error!("Error: {}", e);
            Err(e)
        }
    };
}

async fn heartbeat() -> Result<reqwest::Response, Box<dyn std::error::Error>> {
    let resp = get("http://localhost:3000/heartbeat")
        .await?;

    return Ok(resp);
 }

 async fn update_template() -> Result<reqwest::Response, Box<dyn std::error::Error>> {
    let resp = get("http://localhost:3000/update")
        .await?;

    return Ok(resp);
 }