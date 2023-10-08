// mod v1;
use common::{MetricV1, MetricsResponse};
// use wasm;

use axum::{
    body::Bytes,
    debug_handler,
    extract::State,
    Json,
    response::{Response, IntoResponse},
    routing::{get,post},
    Router,
};
use serde_json::{Value, json};
use serde::{Serialize,Deserialize};
use std::{collections::HashMap, sync::{Arc, RwLock}};
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use chrono::DateTime;
use base64::{engine::general_purpose, Engine as _};

type SharedState = Arc<RwLock<AppState>>;

#[derive(Default)]
struct AppState {
    db: HashMap<String, u32>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
    .with(
        tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "hello-wasm=debug,tower_http=debug,axum=debug".into()),
    )
    .with(tracing_subscriber::fmt::layer())
    .init();

    let shared_state = SharedState::default();

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/stats", get(stats))
        .route("/v1/ingest", post(ingest_metrics_v1))
        // .with_state(Arc::clone(&shared_state))
        .route("/v2/ingest", post(ingest_metrics_v2))
        .with_state(Arc::clone(&shared_state))
        .merge(serve_static());

    // run it with hyper on localhost:3000
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.layer(TraceLayer::new_for_http()).into_make_service())
        .await
        .unwrap();
}

#[debug_handler]
async fn ingest_metrics_v1(State(state): State<SharedState>, body: Bytes) -> Json<Value> {
    println!("ingest_metrics_v1: {:?}", &body);
    let dec = general_purpose::STANDARD.decode(&body).unwrap();
    let dr: hello_wasm::v1::MetricV1 = rmp_serde::from_slice(&dec).unwrap();

    let counter = match state.read().unwrap().db.get(&dr.auth_token) {
        Some(count) => count.clone(),
        None => 0
    };

    println!("{} counter: {}", &dr.auth_token, &counter);

    state.write().unwrap().db.insert(dr.auth_token.clone(), counter + 1);

    Json(json!(dr))
}

#[debug_handler]
async fn ingest_metrics_v2(State(state): State<SharedState>, body: Bytes) -> Json<Value> {
    println!("ingest_metrics_v2: request body: {:?}", &body);
    let dec = general_purpose::STANDARD.decode(&body).unwrap();
    let dr: hello_wasm::v1::MetricV1 = rmp_serde::from_slice(&dec).unwrap();

    let counter = match state.read().unwrap().db.get(&dr.auth_token) {
        Some(count) => count.clone(),
        None => 0
    };

    let new_count = counter + 1;

    state.write().unwrap().db.insert(dr.auth_token.clone(), new_count);

    println!("{} counter: {}", &dr.auth_token, &counter);

    println!("ingest_metrics_v2: response: {:?}", &dr);

    // Json(json!(dr))
    Json(json!(MetricsResponse{status: String::from("ok"), count: new_count}))
}

#[debug_handler]
async fn stats(State(state): State<SharedState>) -> Json<Value> {
    let db = state.read().unwrap().db.clone();

    println!("hashmap; {:?}", &db);

    Json(json!(db))
}

fn serve_static() -> Router {
    let serve_wasm = ServeDir::new("../hello-wasm/dist");

    Router::new()
        .nest_service("/wasm", serve_wasm)
}

/*
    let mut buf = Vec::new();
    let m = Metric{client_id: 1, auth_token: "auth".into(), datetime: chrono::Utc::now()};
    m.serialize(&mut Serializer::new(&mut buf)).unwrap();
 */