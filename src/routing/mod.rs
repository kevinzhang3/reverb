pub mod handlers;
pub mod router;
pub mod not_found;

use std::collections::HashMap;
use hyper::Request;
use hyper::body::Incoming;
use super::response::Response;

pub type Handler = fn(request: Request<Incoming>) -> Response;

pub struct Router {
    debug: bool,
    get_map: HashMap<String, Handler>, 
    static_mounts: Vec<(String, String)>,
}
