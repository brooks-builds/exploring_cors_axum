use axum::{
    http::Request,
    middleware::{from_fn, Next},
    response::Response,
    routing::{get, post, Route},
    RequestExt, Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_world))
        .layer(from_fn(my_cors));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello_world() -> String {
    "Hello World".to_owned()
}

async fn my_cors<B>(request: Request<B>, next: Next<B>) -> Response {
    // do something with `request`...

    let mut response = next.run(request).await;

    let headers = response.headers_mut();
    headers.insert(
        "Access-Control-Allow-Origin",
        "http://localhost:8080".parse().unwrap(),
    );

    headers.insert("Access-Control-Request-Method", "POST".parse().unwrap());

    headers.insert("Access-Control-Allow-Methods", "POST".parse().unwrap());

    headers.insert("Access-Control-Allow-Headers", "*".parse().unwrap());

    headers.insert("Access-Control-Allow-Credentials", "true".parse().unwrap());

    headers.insert(
        "Authorization",
        "Bearer 2389475wufplgjwyu8f7jgwfulpgjfp-q324y5jwfpul.,2yu348j5wfulpghireasnhearhen"
            .parse()
            .unwrap(),
    );

    // do something with `response`...

    response
}
