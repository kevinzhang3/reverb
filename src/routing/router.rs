use futures::future::{BoxFuture, FutureExt};
use http_body_util::Full;
use hyper::body::{Incoming, Bytes};
use hyper::http::{Request, Response as HyperResponse};
use anyhow::{anyhow, Error, Result};
use tracing_subscriber::EnvFilter;
use std::collections::HashMap;
use tokio::net::TcpListener;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use std::sync::Arc;
use hyper_util::rt::TokioIo;
use super::{
    handlers,
    not_found::not_found_response,
    super::response::{Response, build_response},
};
use std::future::Future;
use std::pin::Pin;

type BoxHandler = Box<
    dyn Fn(Request<Incoming>) -> Pin<Box<dyn Future<Output = Response> + Send>>
    + Send
    + Sync
>;

pub struct Router {
    debug: bool,
    get_map: HashMap<String, BoxHandler>,
    post_map: HashMap<String, BoxHandler>,
    static_mounts: Vec<(String, String)>,
}



// for fn pointer mapping 
impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

impl Router {
    pub fn new() -> Self {
        Self {
            debug: false,
            get_map: HashMap::new(),
            post_map: HashMap::new(),
            static_mounts: Vec::new(),
        }
    }

    pub fn debug(mut self, arg: bool) -> Self {
        self.debug = arg;
        self
    }

    pub fn serve_static(mut self, mount_path: &str, dir: &str) -> Self {
        self.static_mounts.push((mount_path.to_string(), dir.to_string()));
        self 
    }

    // insert into map 
    pub fn get<H, F>(mut self, path: &str, handler: H) -> Self
    where
        H: Fn(Request<Incoming>) -> F + Send + Sync + 'static,
        F: Future<Output = Response> + Send + 'static,
        {
            self.get_map.insert(
                path.to_string(),
                Box::new(move |req| Box::pin(handler(req))),
            );
            self
        }


    pub fn post<H, F>(mut self, path: &str, handler: H) -> Self
    where
        H: Fn(Request<Incoming>) -> F + Send + Sync + 'static,
        F: Future<Output = Response> + Send + 'static,
        {
            self.post_map.insert(
                path.to_string(),
                Box::new(move |req| Box::pin(handler(req))),
            );
            self
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
    fn handle(self: Arc<Self>, req: Request<Incoming>) -> BoxFuture<'static, Result<HyperResponse<Full<Bytes>>>> {
        async move {

            // GET
            if let Some(handler) = self.get_map.get(req.uri().path()) {
                let uri = req.uri().path().to_string();
                let resp = build_response(handler(req).await)
                    .await
                    .inspect(|resp| tracing::info!("GET: {:#?} {:#?} {:#?}", uri, resp.status(), resp.version()));
                return resp;
            }
            
            // POST
            if let Some(handler) = self.post_map.get(req.uri().path()) {
                let uri = req.uri().path().to_string();
                let resp = build_response(handler(req).await)
                    .await
                    .inspect(|resp| tracing::info!("POST: {:#?} {:#?} {:#?}", uri, resp.status(), resp.version()));
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn constructor_test() {
        let router = Router::new();
        assert!(!router.debug);
    }

    #[test]
    fn serve_static_test() {
        let router = Router::new()
            .serve_static("test", "/");
        assert_eq!(router.static_mounts.last(), Some(&("test".to_string(), "/".to_string())));
    }


}
