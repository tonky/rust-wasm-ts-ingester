use axum::{
    body::Bytes,
    debug_handler,
    routing::{get,post},
    Router, response::{Response, IntoResponse}, Json,
};
use serde_json::{Value, json};
use serde::{Serialize,Deserialize};
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use rmp_serde::{Deserializer, Serializer};
use chrono::DateTime;
use base64::{engine::general_purpose, Engine as _};


#[tokio::main]
async fn main() {
        tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_static_file_server=debug,tower_http=debug,axum=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/ingest", post(ingest_metrics))
        .merge(serve_static());

    // run it with hyper on localhost:3000
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.layer(TraceLayer::new_for_http()).into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct Metric {
    client_id: u32,
    auth_token: String,
    datetime: DateTime<chrono::Utc>
}

#[debug_handler]
async fn ingest_metrics(body: Bytes) -> Json<Value> {
    println!(">> got body: {:?}", body);
/*
    let mut buf = Vec::new();

    let m = Metric{client_id: 1, auth_token: "auth".into(), datetime: chrono::Utc::now()};

    println!(">> example json: {}", json!(m));

    m.serialize(&mut Serializer::new(&mut buf)).unwrap();

    println!("{:?} - {:?}", m, buf);

    println!("b64: {}", general_purpose::STANDARD.encode(&buf));

    let ds: Metric = rmp_serde::from_slice(&buf).unwrap();
    println!("Deserialized: {:?}", ds);
 */
    let dec = general_purpose::STANDARD.decode(&body).unwrap();
    println!("decoded b64: {:?}", dec);

    // let ds: Metric = rmp_serde::from_slice(&buf).unwrap();
    let dr: Metric = rmp_serde::from_slice(&dec).unwrap();

    // println!("Deserialized example: {:?}", ds);
    println!("Deserialized request: {:?}", dr);

    Json(json!(dr))
}

fn serve_static() -> Router {
    let serve_wasm = ServeDir::new("hello-wasm/dist");

    Router::new()
        .nest_service("/wasm", serve_wasm)
}