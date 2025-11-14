# reverb

Reverb is a simple web framework written in Rust. The goal is to provide an ergonomic and easy to understand API.

## Example

```rust
use reverb::{Router, Response, Request, DataFormat, HttpStatus, body};

fn greet(_req: Request<body::Incoming>) -> Response {
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
```

Currently supports:
* Fully asynchronous server and routing logic
  * Need to add the ability for users to write async handlers
*  GET requests
*  Serving static files
