use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/bar", get(foo_bar));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> String {
    "get Root!".to_string()
}

async fn get_foo() -> String {
    "get Foo".to_string()
}

async fn post_foo() -> String {
    "post Foo".to_string()
}

async fn foo_bar() -> String {
    "get FooBar".to_string()
}
