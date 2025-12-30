use axum::{
    routing::{get,post},
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(root))
    .route("/foo", get(get_foo).post(post_foo))
    .route("/foo/bar", get(foo_bar));


    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    // which calls one of these handlers
    async fn root()-> &'static str {
         "helloRust" 
        }
    async fn get_foo()-> &'static str {
        "get_foo"
    }
    async fn post_foo()-> &'static str {
        "post_foo"
    }
    async fn foo_bar()-> &'static str {
        "foo_bar"
    }

}