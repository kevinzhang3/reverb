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
use crate::handlers;

// for fn pointer mapping 
pub type Handler = fn(request: Request<hyper::body::Incoming>) -> BoxFuture<'static, Result<Response<Full<Bytes>>>>;

// map GET requests to their handlers 
pub struct Router {
    get_map: HashMap<String, Handler>, 
    post_map: HashMap<String, Handler>, 
    static_mounts: Vec<(String, String)>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            // post and delete maps next
            get_map: HashMap::new(),
            post_map: HashMap::new(),
            static_mounts: Vec::new(),
        }
    }

    pub fn serve_static(&mut self, mount_path: &str, dir: &str) {
        self.static_mounts.push((mount_path.to_string(), dir.to_string()));
    }

    // insert into map 
    pub fn method_get(&mut self, path: &str, handler: Handler) {
        self.get_map.insert(path.to_string(), handler);
    }

    // start the server 
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

    // this calls the handler functions 
    fn handle(&self, req: Request<hyper::body::Incoming>) -> BoxFuture<'static, Result<Response<Full<Bytes>>>> {
        eprintln!("Request: {:#?}", req);

        let path = match req.uri().path() {
            "/" => "/index.html",
            v => v,
        };
       
        // rest api handles
        if let Some(handler) = self.get_map.get(path) {
            return handler(req);
        }

        // static file fallback 
        for (mount_url, dir) in &self.static_mounts {
            if path.starts_with(mount_url) {
                let subpath = &path[mount_url.len()..];
                let fs_path = format!("{}/{}", dir, subpath.trim_start_matches('/'));
                return handlers::serve_static_file(fs_path);
            }
        }

        // 404 fallback
        handlers::not_found()
    }
}
