use futures::future::BoxFuture;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::http::{Request, Response};
use anyhow::Result;
use std::collections::HashMap;
use tokio::net::TcpListener;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use std::sync::Arc;
use hyper_util::rt::TokioIo;

pub type Handler = fn(request: Request<hyper::body::Incoming>) -> BoxFuture<'static, Result<Response<Full<Bytes>>>>;

pub struct Router {
    routes: HashMap<String, Handler>, 
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn add(&mut self, path: &str, handler: Handler) {
        self.routes.insert(path.to_string(), handler);
    }

    pub fn handle(&self, req: Request<hyper::body::Incoming>) -> BoxFuture<'static, Result<Response<Full<Bytes>>>> {
        eprintln!("Request: {:?}", req);

        if let Some(handler) = self.routes.get(req.uri().path()) {
            handler(req)
        } else {
            Box::pin(async {
                Ok(Response::builder()
                    .status(404)
                    .body(Full::new(Bytes::from("Not Found")))
                    .unwrap())
            })
        } 
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
}
