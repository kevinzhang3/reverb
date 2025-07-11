use anyhow::Result;
use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use hyper::http::{Request, Response};
use tokio::fs;
use futures::future::{BoxFuture, FutureExt};


pub fn not_found(req: Request<Incoming>) -> BoxFuture<'static, Result<Response<Full<Bytes>>>> {
    async move {
        eprint!("REQ: {:#?} {:#?}", req.method(), req.uri());
        let json_data = r#"{
            "message": "Not found",
            "status": 404,
        }"#;

        let response = Response::builder()
            .header("Content-Type", "application/json")
            .status(404)
            .body(Full::new(Bytes::from(json_data)))?;
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

pub fn get_json(_req: Request<Incoming>) -> BoxFuture<'static, Result<Response<Full<Bytes>>>> {
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


