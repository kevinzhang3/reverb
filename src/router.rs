use futures::future::BoxFuture;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::http::{Request, Response};
use anyhow::Result;
use std::collections::HashMap;

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
}
