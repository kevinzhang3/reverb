use anyhow::Result;
use hyper::body::Incoming;
use reverb::{
    routing::Router,
    response::Response,
    util::{DataFormat, HttpStatus}
};

fn greet(_req: hyper::Request<Incoming>) -> Response {
    return Response::new(DataFormat::JSON("Hello, world!".to_string()), HttpStatus::Ok); 
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut router = Router::new();

    router.serve_static("/", "public");
    router.get("/greet", greet);
    router.debug(true);
    router.start("127.0.0.1:8080").await?;

    Ok(())
}
