use anyhow::Result;
use futures::{future::BoxFuture, FutureExt};
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::http::Response as HyperResponse;
use super::util::{DataFormat, HttpStatus};

pub fn build_response(format: DataFormat, status: HttpStatus, data: hyper::body::Bytes) -> BoxFuture<'static, Result<HyperResponse<Full<Bytes>>>> {
    async move {
        let response = match format {
            DataFormat::JSON(_) => {
                HyperResponse::builder()
                    .header("Content-Type", "application/json")
                    .status(status.as_status_code())
                    .body(Full::new(data))
            },
            DataFormat::XML(_) => {
                HyperResponse::builder()
                    .header("Content-Type", "application/xml")
                    .status(status.as_status_code())
                    .body(Full::new(data))
            },
        };
        Ok(response?)
    }.boxed()
}

pub struct Response {
    status: HttpStatus,
    format: DataFormat,
    body: String,
}

impl Response {
    pub fn new(format: DataFormat, status: HttpStatus, body: String) -> Self {
        Self {
            status: status,
            format: format,
            body: body,
        }
    }

    pub fn set_status(&mut self, status: HttpStatus) {
        self.status = status;
    }
    
    pub fn set_format(&mut self, format: DataFormat) {
        self.format = format;
    }
    
    pub fn set_data(&mut self, body: String) {
        self.body = body;
    }
}
