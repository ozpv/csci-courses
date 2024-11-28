use axum::{
    http::{header, Method},
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use csci_courses::compile::compile;
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
};

// TODO: better error handling
// and logging

/// htmx responses require html
#[allow(dead_code)]
async fn test_api() -> impl IntoResponse {
    Html("Hello!")
}

/// handles serving htmx dir
#[tokio::main]
async fn main() {
    // TODO: use env variables to set port and site_root
    // port to listen on
    let port = std::env::var("PORT")
        .expect("Please specifiy port in environment")
        .parse::<u16>()
        .expect("PORT must be a valid unsigned 16-bit number");
    // dir of frontend + other files
    // FOR DEBUG BUILDS (cargo watch -x run): relative to the Cargo.toml file
    let site_root = "./htmx";

    // build our routes
    let routes = Router::new()
        // api routes
        .route("/api", get(test_api))
        .route("/api/compile", post(compile))
        .nest_service(
            "/",
            ServeDir::new(site_root)
                .not_found_service(ServeFile::new(format!("{site_root}/404.html"))),
        )
        // enable gzip compression
        .layer(CompressionLayer::new())
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST])
                // important for sending Json
                .allow_headers([header::CONTENT_TYPE]),
        );

    // ideally, use nginx as a reverse proxy when deployed
    let addr = format!("127.0.0.1:{port}");
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Listening on http://{addr}");

    axum::serve(listener, routes).await.unwrap();
}
