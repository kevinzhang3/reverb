use futures::future::BoxFuture;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::http::{Request, Response};
use anyhow::{Result, Context};
use std::collections::HashMap;
use tokio::net::TcpListener;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use std::sync::Arc;
use hyper_util::rt::TokioIo;
use tokio::fs;

pub type Handler = fn(request: Request<hyper::body::Incoming>) -> BoxFuture<'static, Result<Response<Full<Bytes>>>>;

pub struct Router {
    get_map: HashMap<String, Handler>, 
}

impl Router {
    pub fn new() -> Self {
        Self {
            get_map: HashMap::new(),
        }
    }

    pub fn method_get(&mut self, path: &str, handler: Handler) {
        self.get_map.insert(path.to_string(), handler);
    }

    pub async fn start(self, port: &str) -> Result<()> {
        let router = std::sync::Arc::new(self);

        let listener = TcpListener::bind(port).await?;
        println!("Server running on http://{}", port);

        tokio::select! {
            _ = async {
                loop {
                    let (stream, _) = listener.accept().await?;
                    let io = TokioIo::new(stream);
                    let clone = Arc::clone(&router);

                    tokio::task::spawn(async move {
                        if let Err(err) = http1::Builder::new()
                            .keep_alive(true)
                                .serve_connection(io, service_fn(move |req| clone.handle(req)))
                                .await
                        {
                            eprintln!("Error serving connection: {:#?}", err);
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

    fn handle(&self, req: Request<hyper::body::Incoming>) -> BoxFuture<'static, Result<Response<Full<Bytes>>>> {
        eprintln!("Request: {:#?}", req);

        if let Some(handler) = self.get_map.get(req.uri().path()) {
            handler(req)
        } else {
            Box::pin(async {
                let contents = fs::read_to_string("404.html").await
                    .context("Failed to read HTML")?;
                let body = Full::new(Bytes::from(contents));

                let response = Response::builder()
                    .status(404)
                    .body(body)
                    .context("Failed to build response")?;

                Ok(response)
            })
        } 
    }
}
