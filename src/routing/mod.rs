pub mod handlers;
pub mod router;

use std::collections::HashMap;
use hyper::{Request, Response};
use hyper::body::{Incoming, Bytes};
use futures::future::BoxFuture;
use http_body_util::Full;
use anyhow::Result;

pub type Handler = fn(request: Request<Incoming>) -> BoxFuture<'static, Result<Response<Full<Bytes>>>>;

pub struct Router {
    debug: bool,
    get_map: HashMap<String, Handler>, 
    static_mounts: Vec<(String, String)>,
}
