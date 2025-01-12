use axum::{
    http::{header, Method},
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use csci_courses::api;
use std::path::PathBuf;
use std::time::Duration;
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

struct Env {
    dist_dir: PathBuf,
    site_addr: String,
}

impl Env {
    fn get_or_default() -> Self {
        let dist_dir = std::env::var("SITE_ADDR")
            .unwrap_or_else(|_| format!("{}/", env!("CARGO_MANIFEST_DIR")))
            .into();

        let site_addr = std::env::var("SITE_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());

        Self {
            dist_dir,
            site_addr,
        }
    }
}

/// htmx responses require html
async fn hello_from_axum() -> impl IntoResponse {
    Html("Hello!")
}

async fn json_example() {
    use csci_courses::compiler::{FileData, SourceFile};
    let source = SourceFile::Cpp(FileData::new(
        "main".to_string(),
        String::from_utf8(include_bytes!("../example/example_assignment.cpp").to_vec()).unwrap(),
    ));
    println!(
        "{}",
        serde_json::to_string(&api::CompileOptions {
            source_code: vec![source]
        })
        .unwrap()
    );
}

/// handles serving htmx dir
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    json_example().await;
    let Env {
        dist_dir,
        site_addr,
    } = Env::get_or_default();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let error_file = ServeFile::new(format!("{}/404.html", dist_dir.display()));
    // build our routes
    let routes = Router::new()
        // api routes
        .route("/api", get(hello_from_axum))
        .route("/api/compile", post(api::compile))
        .fallback_service(
            ServeDir::new(dist_dir.join("htmx")).not_found_service(error_file.clone()),
        )
        .nest_service(
            "/assets",
            ServeDir::new(dist_dir.join("assets")).not_found_service(error_file),
        )
        .layer(CompressionLayer::new().gzip(true))
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST])
                .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE]),
        )
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(&site_addr).await.unwrap();

    tracing::info!("Listening on http://{site_addr}/");
    axum::serve(listener, routes).await?;

    Ok(())
}
