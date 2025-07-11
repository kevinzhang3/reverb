use anyhow::Result;
use futures::{future::BoxFuture, FutureExt};
use http_body_util::Full;
use hyper::{body::Bytes, Response};

pub enum HttpStatus {
    Ok = 200,
    NotFound = 404,
}

pub enum DataFormat {
    JSON(Bytes),
    XML(Bytes),
}

pub fn get_wrap<T>(response: (DataFormat, HttpStatus)) -> BoxFuture<'static, Result<Response<Full<Bytes>>>> {
    async move {
        let wrap = match response.0 {
            DataFormat::JSON(json) => {
                Response::builder()
                    .header("Content-Type", "application/json")
                    .status(200)
                    .body(Full::new(json))
            },
            DataFormat::XML(xml) => {
                Response::builder()
                    .header("Content-Type", "application/xml")
                    .status(200)
                    .body(Full::new(xml))
            },
        };
        Ok(wrap?)
    }.boxed()
}
