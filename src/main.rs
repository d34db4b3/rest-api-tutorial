use axum::{
    http::{self, HeaderValue, Response, StatusCode},
    routing::get,
    Router,
};
use std::{any::Any, net::SocketAddr, ptr};
use tower_http::catch_panic::{CatchPanicLayer, ResponseForPanic};

async fn segmentation_fault() {
    unsafe {
        let _ = ptr::null::<u8>().read();
    }
    unreachable!()
}

async fn sig_kill() {
    nix::sys::signal::kill(nix::unistd::getpid(), nix::sys::signal::SIGKILL).unwrap();
    unreachable!()
}

async fn panic() {
    panic!()
}

#[derive(Clone)]
struct PanicHandler;

impl ResponseForPanic for PanicHandler {
    type ResponseBody = String;

    fn response_for_panic(
        &mut self,
        err: Box<dyn Any + Send + 'static>,
    ) -> Response<Self::ResponseBody> {
        let mut res = Response::new(format!(
            "Panic: {}",
            if let Some(s) = err.downcast_ref::<String>() {
                s.as_str()
            } else if let Some(s) = err.downcast_ref::<&str>() {
                s
            } else {
                "???"
            }
        ));
        *res.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;

        const TEXT_PLAIN: HeaderValue = HeaderValue::from_static("text/plain; charset=utf-8");
        res.headers_mut()
            .insert(http::header::CONTENT_TYPE, TEXT_PLAIN);

        res
    }
}

#[tokio::main]
async fn main() {
    // register our "greeting" handler to the `/greet` route
    let app = Router::new()
        .route("/panic", get(panic))
        .route("/sigkill", get(sig_kill))
        .route("/segfault", get(segmentation_fault))
        .layer(CatchPanicLayer::custom(PanicHandler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on http://{}", addr);
    // start the HTTP server and serve our newly created service
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
