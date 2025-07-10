use anyhow::{Context, Result};
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::http::{Request, Response};
use tokio::fs;
use hyper::body::Incoming;
use futures::future::{BoxFuture, FutureExt};


pub fn not_found() -> BoxFuture<'static, Result<Response<Full<Bytes>>>> {
    async move {
        let contents = fs::read_to_string("public/404.html")
            .await
            .context("Failed to read HTML")?;
        let body = Full::new(Bytes::from(contents));

        let response = Response::builder()
            .header("Content-Type", "text/html")
            .status(404)
            .body(body)
            .context("Failed to build response")?;

        Ok(response)
    }.boxed()
}

pub fn serve_static_file(path: String) -> BoxFuture<'static, Result<Response<Full<Bytes>>>> {
    async move {
        match fs::read(&path).await {
            Ok(data) => {
                let mime = mime_guess::from_path(&path).first_or_octet_stream();
                let response = Response::builder()
                    .header("Content-Type", mime.to_string())
                    .status(200)
                    .body(Full::new(Bytes::from(data)))?;
                Ok(response)
            },
            Err(_) => {
                not_found().await
            }
        }
    }.boxed()
}

pub fn get_json(_req: &Request<Incoming>) -> BoxFuture<'static, Result<Response<Full<Bytes>>>> {
    async move {
        
        let json_data = r#"{
            "message": "GET: JSON API TEST",
            "status": 200,
            "items": [1, 2, 3]
        }"#;

        let response = Response::builder()
            .status(200)
            .header("Content-Type", "application/json")
            .body(Full::new(Bytes::from(json_data)))?;

        Ok(response)
    }.boxed()
}


