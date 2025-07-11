use anyhow::Result;
use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use hyper::http::{Request, Response};
use tokio::fs;
use futures::future::{BoxFuture, FutureExt};


pub fn not_found(req: Request<Incoming>) -> BoxFuture<'static, Result<Response<Full<Bytes>>>> {
    async move {
        tracing::error!("REQ: {:#?}", req);
        let json_data = r#"{
            "message": "Not found",
            "status": 404,
        }"#;

        let resp = Response::builder()
            .header("Content-Type", "application/json")
            .status(404)
            .body(Full::new(Bytes::from(json_data)))?;
        tracing::error!("RESP: {:#?} {:#?}", resp.status(), resp.version());
        Ok(resp)
    }.boxed()
}

pub fn serve_static_file(req: Request<Incoming>, mount_url: String, dir: String) -> BoxFuture<'static, Result<Response<Full<Bytes>>>> {
    async move {
        
        let path = match req.uri().path() {
            "/" => "/index.html",
            v => v,
        };
        
        let subpath = &path[mount_url.len()..];
        let fs_path = format!("{}/{}", dir, subpath.trim_start_matches('/'));

        match fs::read(&fs_path).await {
            Ok(data) => {
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                let response = Response::builder()
                    .header("Content-Type", mime.to_string())
                    .status(200)
                    .body(Full::new(Bytes::from(data)))?;
                Ok(response)
            },
            Err(_) => {
                not_found(req).await
            }
        }
    }.boxed()
}

