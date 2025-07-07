use std::convert::Infallible;
use std::fs;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (stream, _) = listener.accept().await?;

        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .keep_alive(true)
                .serve_connection(io, service_fn(router))
                .await
            {
                eprint!("Error serving connection: {:?}", err);
            }
        });
    }
}

async fn router(request: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    eprint!("Request: {:#?}", request);

    let response = match request.uri().path() {
        "/" => Response::builder()
            .status(200)
            .header("foo", "bar")
            .body(Full::new(Bytes::from(fs::read_to_string("rust.html").unwrap())))
            .unwrap(),
        _ => Response::builder()
            .status(404)
            .body(Full::new(Bytes::from(fs::read_to_string("404.html").unwrap())))
            .unwrap()
    };

    Ok(response)
}
