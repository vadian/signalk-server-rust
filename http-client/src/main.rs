use reqwest::get;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    //let result = heartbeat().await;
    //let result = update().await;
    let bucket = "signalk_events";

    let q: String = format!("from(bucket: \"{}\") 
    |> range(start: -1w)
    |> filter(fn: (r) => r.path == \"{}\") 
    |> last()
    ", bucket, "http_client");

    let result = query(q).await;

    return match result {
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
    };
}

async fn heartbeat() -> Result<reqwest::Response, reqwest::Error> {
    let resp = get("http://localhost:3000/heartbeat")
        .await?;

    return resp.error_for_status();
 }

 async fn query(query: String) -> Result<reqwest::Response, reqwest::Error> {
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
        time: chrono::offset::Utc::now(),
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