use reqwest::{get, Response};
use signalk_core::DateTimeTz;
use anyhow::{Result, anyhow};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    match process_result(update().await).await {
    Ok(()) => println!("Successful update"),
    Err(e) => panic!("{}", e.to_string()),
}

    return process_result(query().await).await;
}

async fn process_result(resp: Result<Response, reqwest::Error>) -> Result<()> {
    match resp {
        Ok(resp) => {
            tracing::debug!("Good result: {}", resp.status());
            let text = resp.text().await?;
            tracing::debug!("Contents: {}", text);
            Ok(())
        },
        Err(e) => {
            tracing::error!("Error: {}", e);
            Err(e.into())
        }
    }
}

async fn heartbeat() -> Result<Response> {
    let resp = get("http://localhost:3000/heartbeat")
        .await?;

    return match resp.error_for_status() {
        Ok(v) => Ok(v),
        Err(e) => Err(anyhow!(e)),
    }
 }

 async fn query() -> Result<reqwest::Response, reqwest::Error> {
    let query = "foo";
    let resp = get(format!("http://localhost:3000/query/{}", query))
        .await?;

    return resp.error_for_status();
 }

 async fn update_template() -> Result<reqwest::Response, Box<dyn std::error::Error>> {
    let resp = get("http://localhost:3000/update")
        .await?;

    return Ok(resp);
 }

 async fn update() -> Result<reqwest::Response, reqwest::Error> {
    let update = ::signalk_core::Update{        
        timestamp: DateTimeTz(chrono::Utc::now().with_timezone(&chrono_tz::UTC)),
        path: String::from("boat.name"),
        value: String::from("Boaty McBoatface"),
        source: String::from("http_client")
    };
    
    let resp = reqwest::Client::new()
        .post("http://localhost:3000/update")
        .json(&update)
        .send()
        .await?;

    return resp.error_for_status();
 }