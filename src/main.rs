use axum::{extract::Query, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Deserialize)]
struct GreetQuery {
    name: Option<String>,
}

#[derive(Serialize)]
struct GreetResponse {
    greeting: String,
}

async fn greet(greet_query: Query<GreetQuery>) -> Json<GreetResponse> {
    Json(GreetResponse {
        greeting: format!(
            "Hello, {}!",
            greet_query
                .name
                .as_ref()
                .map(String::as_str)
                .unwrap_or("Anonymous")
        ),
    })
}

#[tokio::main]
async fn main() {
    // register our "greeting" handler to the `/greet` route
    let app = Router::new().route("/greet", get(greet));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on http://{}", addr);
    // start the HTTP server and serve our newly created service
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
