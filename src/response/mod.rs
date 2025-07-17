use anyhow::Result;
use futures::{future::BoxFuture, FutureExt};
use http_body_util::Full;
use hyper::body::{Bytes, Response};
use super::util::{DataFormat, HttpStatus};

pub fn get_wrap(response: (DataFormat, HttpStatus)) -> BoxFuture<'static, Result<Response<Full<Bytes>>>> {
    async move {
        let status = response.1.as_status_code();
        let wrap = match response.0 {
            DataFormat::JSON(json) => {
                Response::builder()
                    .header("Content-Type", "application/json")
                    .status(status)
                    .body(Full::new(json))
            },
            DataFormat::XML(xml) => {
                Response::builder()
                    .header("Content-Type", "application/xml")
                    .status(status)
                    .body(Full::new(xml))
            },
        };
        Ok(wrap?)
    }.boxed()
}
