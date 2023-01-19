use axum::{
    extract,
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use influxdb2::{Client, FromDataPoint, models::{DataPoint, Status}};
use influxdb2::models::Query;
use chrono::{DateTime, Utc, FixedOffset, TimeZone};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/heartbeat", get(health_check))
        .route("/update", post(update))
        .route("/update", get(update_template))
        .route("/query/:query", get(query));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn health_check() -> impl IntoResponse {

    let client = Client::new("http://localhost:8086", "default", "");

    let foo = client.ready().await;
    
    let ret = match foo {
        Ok(str) => (StatusCode::OK, Json(str.to_string())),
        Err(err) => (StatusCode::EXPECTATION_FAILED, Json(err.to_string())),
    };

    return ret;
}

async fn update(
    Json(payload): Json<Update>
) -> impl IntoResponse {
    let token = "n6SbaAaX02RxTplLOFf2oB7079nxOkYlvKXmxlK35wcJ1w-TT1PxLqr0nuvGLa-ntZRvxhi4i0LduHLnD8AcEQ==";
    let client = Client::new("http://localhost:8086", "default", token);
    let bucket = "signalk_events";

    let update = DataPoint::builder(payload.path)
            .tag("source", payload.source)
            .field("value", payload.value)
            .build();

    let validated_update = match update {
        Ok(p) => vec!(p),
        Err(e) => return (StatusCode::BAD_REQUEST, e.to_string())
    };

    let foo = client.write(bucket, futures::stream::iter(validated_update)).await;
    
    return match foo {
        Ok(_) => (StatusCode::CREATED, "Update successful.".to_string()),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string())
    };
}

async fn query(extract::Path(req): extract::Path<String>) -> Result<Json<Vec<Update>>, StatusCode> {
    let token = "n6SbaAaX02RxTplLOFf2oB7079nxOkYlvKXmxlK35wcJ1w-TT1PxLqr0nuvGLa-ntZRvxhi4i0LduHLnD8AcEQ==";
    let client = Client::new("http://localhost:8086", "default", token);
    let bucket = "signalk_events";

    let query = Query::new(req);
    let res = client.query::<Update>(Some(query))
        .await;

    match res  {
        Ok(v) => Result::Ok(Json(v)),
        Err(e) => Result::Err(StatusCode::BAD_REQUEST)
    }        

}

async fn update_template() -> impl IntoResponse {
    let offset = FixedOffset::east(0);
    let template = Update {
        time: Utc::now().with_timezone(&offset),
        path: String::from("/path/to"),
        value: String::from("new_value"),
        source: String::from("source_identifier")
    };

    (StatusCode::OK, Json(template))
}

#[derive(Debug, FromDataPoint)]
#[derive(Deserialize, Serialize)]
struct Update{
    time: DateTime<FixedOffset>,
    path: String,
    source: String,
    value: String,
}

impl Default for Update {
    fn default() -> Self {
        let offset = FixedOffset::east(0);
        Self {
            path: "".to_string(),
            source: "".to_string(),
            value: "".to_string(),
            time: offset.from_utc_datetime(&DateTime::<Utc>::MIN_UTC.naive_utc()),
        }
    }
}