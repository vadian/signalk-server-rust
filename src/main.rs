use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, ops::Add};
use influxdb::{Client, Query, Timestamp, ReadQuery};
use influxdb::InfluxDbWriteable;
use chrono::{DateTime, Utc};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/heartbeat", get(health_check))
        .route("/update", post(update))
        .route("/update", get(update_template));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn health_check() -> impl IntoResponse {

    let client = Client::new("http://localhost:8086", "default");

    let foo = client.ping().await;
    
    let ret = match foo {
        Ok(str) => (StatusCode::OK, Json(str)),
        Err(err) => (StatusCode::EXPECTATION_FAILED, Json(("ERROR".to_string(), err.to_string()))),
    };

    return ret;
}

async fn update(
    Json(payload): Json<Update>
) -> impl IntoResponse {
    
    //todo: save in db

    (StatusCode::CREATED, Json(payload))
}

async fn update_template() -> impl IntoResponse {
    let template = Update {
        delta: Delta {
            time: chrono::offset::Utc::now(),
            path: String::from("/path/to"),
            value: String::from("new_value")
        },
        source: Source {
            id: String::from("source_identifier")
        }
    };

    (StatusCode::OK, Json(template))
}

#[derive(InfluxDbWriteable)]
#[derive(Deserialize, Serialize)]
struct Delta {
    path: String,
    value: String,
    time: DateTime<Utc>,
}

#[derive(Deserialize, Serialize)]
struct Update{
    delta: Delta,
    source: Source,
}

#[derive(Deserialize, Serialize)]
struct Source {
    id: String,
}