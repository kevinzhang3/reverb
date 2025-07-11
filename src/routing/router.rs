use futures::future::{BoxFuture, FutureExt};
use http_body_util::Full;
use hyper::body::{Incoming, Bytes};
use hyper::http::{Request, Response};
use anyhow::Result;
use std::collections::HashMap;
use tokio::net::TcpListener;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use std::sync::Arc;
use hyper_util::rt::TokioIo;
use super::handlers;

// for fn pointer mapping 
pub type Handler = fn(request: Request<Incoming>) -> BoxFuture<'static, Result<Response<Full<Bytes>>>>;

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
        let router = Arc::new(self);

        let listener = TcpListener::bind(port).await?;
        println!("Server running on http://{}", port);

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
                    eprintln!("ERR: {:#?}", e);
               }
            });
        }
    }

    // this calls the handler functions 
    fn handle(self: Arc<Self>, req: Request<Incoming>) -> BoxFuture<'static, Result<Response<Full<Bytes>>>> {
        async move {

            // rest api handles
            if let Some(handler) = self.get_map.get(req.uri().path()) {
                eprint!("REQ: {:#?} {:#?} | ", req.method(), req.uri());
                
                let resp = handler(req)
                    .await
                    .inspect(|resp| eprintln!("RESP: {:#?} {:#?}", resp.status(), resp.version()));
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
            handlers::not_found(req).await
        }.boxed()
    }
}
