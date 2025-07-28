use reverb::{
    routing::Router,
    response::Response,
    util::{DataFormat, HttpStatus}
};

fn greet(_req: hyper::Request<hyper::body::Incoming>) -> Response {
    return Response::new(DataFormat::JSON("Hello, world!".to_string()), HttpStatus::Ok); 
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .serve_static("/", "public")
        .get("/greet", greet)
        .debug(true);

    router.start("127.0.0.1:8080").await.unwrap();
}
