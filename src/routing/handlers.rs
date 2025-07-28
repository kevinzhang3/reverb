use anyhow::{anyhow, Result, Error};
use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use hyper::http::{Request, Response};
use tokio::fs;
use futures::future::{BoxFuture, FutureExt};
use super::not_found::not_found_response;

pub fn serve_static_file(req: Request<Incoming>, mount_url: String, dir: String) -> BoxFuture<'static, Result<Response<Full<Bytes>>>> {
    async move {
        
        let path = match req.uri().path() {
            "/" => "/index.html",
            v => v,
        };
        
        let root = env!("CARGO_MANIFEST_DIR");
        let subpath = &path[mount_url.len()..];
        let fs_path = format!("{}/{}/{}", root, dir, subpath.trim_start_matches('/'));

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
                let err: Error = anyhow!("ERR: INVALID URI");
                not_found_response(req, err).await
            }
        }
    }.boxed()
}

