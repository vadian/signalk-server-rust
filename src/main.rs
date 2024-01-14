use axum::{
    extract::{self, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use emseries::{DateTimeTz, Record, Series};

use signalk_core::Update;
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono_tz;

static FILE: &str = ".\\db.series.ndjson";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let series = match Series::<Update>::open(FILE) {
        Ok(str) => str,
        Err(err) => panic!("{}", err),
    };

    let state = Arc::new(Mutex::new(series));
    let app = Router::new()
        .route("/heartbeat", get(health_check))
        .route("/update", post(update))
        .route("/update", get(update_template))
        .route("/query/:query", get(query))
        .with_state(state);

    tracing::debug!("listening on 0.0.0.0:3000");
    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app,
    )
    .await
    .unwrap();
}

async fn health_check(State(_state): State<Arc<Mutex<Series<Update>>>>) -> impl IntoResponse {
    match Series::<Update>::open(FILE) {
        Ok(_str) => (StatusCode::OK, Json("{}".to_string())),
        Err(err) => (StatusCode::EXPECTATION_FAILED, Json(err.to_string())),
    }
}

async fn update(
    State(state): State<Arc<Mutex<Series<Update>>>>,
    Json(payload): Json<Update>,
) -> impl IntoResponse {
    let res = state.lock().await.put(payload);

    match res {
        Ok(p) => (StatusCode::CREATED, Json(p.to_string())),
        Err(e) => (StatusCode::BAD_REQUEST, Json(e.to_string())),
    }
}

async fn query(
    State(state): State<Arc<Mutex<Series<Update>>>>,
    extract::Path(req): extract::Path<String>,
) -> Result<Json<Vec<Record<Update>>>, StatusCode> {
    println!("Request: {}", req);
    let res = state.lock().await.all_records();

    match res {
        Ok(v) => Ok(axum::Json(v.to_vec())),
        Err(_e) => Result::Err(StatusCode::BAD_REQUEST),
    }
}

async fn update_template() -> impl IntoResponse {
    let template = Update {
        path: String::from("/path/to"),
        value: String::from("new_value"),
        source: String::from("source_identifier"),
        timestamp: utc_now(),
    };

    (StatusCode::OK, Json(template))
}

pub fn utc_now() -> DateTimeTz {
    DateTimeTz(chrono::Utc::now().with_timezone(&chrono_tz::UTC))
}