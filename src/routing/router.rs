use futures::future::{BoxFuture, FutureExt};
use http_body_util::Full;
use hyper::body::{Incoming, Bytes};
use hyper::http::{Request, Response};
use anyhow::{anyhow, Error, Result};
use tracing_subscriber::EnvFilter;
use std::collections::HashMap;
use tokio::net::TcpListener;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use std::sync::Arc;
use hyper_util::rt::TokioIo;
use super::handlers;
use super::{Router, Handler};
use super::not_found::not_found_response;

// for fn pointer mapping 
impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

impl Router {
    pub fn new() -> Self {
        Self {
            // post and delete maps next
            debug: false,
            get_map: HashMap::new(),
            static_mounts: Vec::new(),
        }
    }

    pub fn debug(&mut self, arg: bool) {
        self.debug = arg;
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
        if self.debug { 
            tracing_subscriber::fmt()
                .with_env_filter(EnvFilter::new("info"))
                .with_target(false)
                .init();
        }

        let router = Arc::new(self);

        let listener = TcpListener::bind(port).await?;
        eprintln!("Server running on http://{}", port);

        loop {
            let (stream, _) = listener.accept().await?;
            let io = TokioIo::new(stream);
            let clone = Arc::clone(&router);

            tokio::task::spawn(async move {
                if let Err(e) = http1::Builder::new()
                    .keep_alive(true)
                        .serve_connection(io, service_fn(move |req| {
                            let router = Arc::clone(&clone);
                            router.handle(req)
                        })).await
                {
                    tracing::error!("ERR: {:#?}", e);
                }
            });
        }
    }

    // this calls the handler functions 
    fn handle(self: Arc<Self>, req: Request<Incoming>) -> BoxFuture<'static, Result<Response<Full<Bytes>>>> {
        async move {

            // rest api handles
            if let Some(handler) = self.get_map.get(req.uri().path()) {
                tracing::info!("REQUEST: {:#?}", req);

                let resp = handler(req)
                    .await
                    .inspect(|resp| tracing::info!("RESPONSE: {:#?} {:#?}", resp.status(), resp.version()));
                return resp;
            }

            // static file fallback 
            for (mount_url, dir) in &self.static_mounts {
                let path = req.uri().path();
                if path.starts_with(mount_url) {
                    return handlers::serve_static_file(req, mount_url.to_string(), dir.to_string()).await;
                }
            }

            // 404 fallback
            let err: Error = anyhow!("ERR: INVALID URI");
            not_found_response(req, err).await
        }.boxed()
    }
}
