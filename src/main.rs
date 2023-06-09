use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // register a "helloworld" handler to the `/` route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on http://{}", addr);
    // start the HTTP server and serve our newly created service
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
