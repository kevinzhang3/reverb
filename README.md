# reverb

Reverb is a minimal web framework that provides a router, server, and the ability to write custom REST API endpoints. 

## Example

```rust
use reverb::{Router, Response, Request, DataFormat, HttpStatus, body, BodyExt};

async fn greet(_req: Request<body::Incoming>) -> Response {
    return Response::new(DataFormat::JSON("Hello, world!".to_string()), HttpStatus::Ok); 
}

async fn return_body(req: Request<body::Incoming>) -> Response {
    let body = req.collect().await.unwrap().to_bytes();
    let str = String::from_utf8(body.to_vec()).unwrap();
    return Response::new(DataFormat::JSON(str), HttpStatus::Ok);
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .serve_static("/", "public")
        .get("/greet", greet)
        .post("/post", return_body)
        .debug(true);

    router.start("127.0.0.1:8080").await.unwrap();
}
```


