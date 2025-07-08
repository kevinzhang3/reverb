use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::http::{Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use anyhow::{Result, Context};
use tokio::fs;
use std::time::Instant;

// default runtime uses the N threads where N = num of cores
#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server running on http://127.0.0.1:8080");

    tokio::select! {
        _ = async {
            loop {
                let (stream, _) = listener.accept().await?;
                let io = TokioIo::new(stream);

                tokio::task::spawn(async move {
                    if let Err(err) = http1::Builder::new()
                        .keep_alive(true)
                        .serve_connection(io, service_fn(router))
                        .await
                    {
                        eprintln!("Error serving connection: {:?}", err);
                    }
                });
            }

            #[allow(unreachable_code)]
            Ok::<_, anyhow::Error>(())
        } => {},

        _ = tokio::signal::ctrl_c() => {
            println!("\nShutting down.");
        }
    }

    Ok(())
}

async fn router(request: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>> {
    eprintln!("{:#?}", request);

    let response = match request.uri().path() {
        "/" => base_uri().await?,
        _ => not_found().await?
    };

    eprintln!(
        "Handled {} in {:.2?}",
        request.uri().path(),
        duration
    );

    Ok(response)
}

async fn base_uri() -> Result<Response<Full<Bytes>>> {
    let contents = fs::read_to_string("rust.html").await
        .context("Failed to read HTML")?;
    let body = Full::new(Bytes::from(contents));

    let response = Response::builder()
        .status(200)
        .body(body)
        .context("Failed to build response")?;
    
    Ok(response)
}

async fn not_found() -> Result<Response<Full<Bytes>>> {
    let contents = fs::read_to_string("404.html").await
        .context("Failed to read HTML")?;
    let body = Full::new(Bytes::from(contents));

    let response = Response::builder()
        .status(404)
        .body(body)
        .context("Failed to build response")?;

    Ok(response)
}
