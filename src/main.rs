use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

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

async fn health_check() -> &'static str {
    "SignalK Server OK"
}

async fn update(
    Json(payload): Json<Update>
) -> impl IntoResponse {
    
    //todo: save in db
    
    (StatusCode::CREATED, Json(payload.delta))
}

async fn update_template() -> impl IntoResponse {
    let template = Update {
        delta: Delta {
            path: "/path/to".to_string(),
            value: "new_value".to_string()
        },
        source: Source {
            id: "source_identifier".to_string()
        }
    };

    (StatusCode::OK, Json(template))
}

#[derive(Deserialize, Serialize)]
struct Delta {
    path: String,
    value: String,
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