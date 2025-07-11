use anyhow::{Result, Error};
use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use hyper::http::{Request, Response};
use futures::future::{BoxFuture, FutureExt};

pub fn not_found_response(req: Request<Incoming>, err: Error) -> BoxFuture<'static, Result<Response<Full<Bytes>>>> {
    async move {
        tracing::error!("REQ: {:#?} {:#?} | ERR: {:?}", req.method(), req.uri(), err);
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
