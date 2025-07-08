use anyhow::{Context, Result};
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::http::{Response, Request};
use tokio::fs;
use futures::future::{BoxFuture, FutureExt};


pub fn base_uri(_req: Request<hyper::body::Incoming>) -> BoxFuture<'static, Result<Response<Full<Bytes>>>> {
    async move {
        let contents = fs::read_to_string("rust.html")
            .await
            .context("Failed to read HTML")?;
        let body = Full::new(Bytes::from(contents));

        let response = Response::builder()
            .status(200)
            .body(body)
            .context("Failed to build response")?;

        Ok(response)
    }.boxed()
}

pub fn serve_static_file(path: String) -> BoxFuture<'static, Result<Response<Full<Bytes>>>> {
    async move {
        let data = fs::read(&path).await.map_err(|e| {
            eprintln!("File not found: {}", path);
            e
        })?;

        let mime = mime_guess::from_path(&path).first_or_octet_stream();
        let response = Response::builder()
            .header("Content-Type", mime.to_string())
            .status(200)
            .body(Full::new(Bytes::from(data)))?;

        Ok(response)
    }.boxed()
}
