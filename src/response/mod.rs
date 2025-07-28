use anyhow::Result;
use futures::{future::BoxFuture, FutureExt};
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::http::Response as HyperResponse;
use super::util::{DataFormat, HttpStatus};

pub fn build_response(context: Response) -> BoxFuture<'static, Result<HyperResponse<Full<Bytes>>>> {
    async move {
        let response = match context.format {
            DataFormat::JSON(body) => {
                HyperResponse::builder()
                    .header("Content-Type", "application/json")
                    .status(context.status.as_status_code())
                    .body(Full::new(Bytes::from(body)))
            },
            DataFormat::XML(body) => {
                HyperResponse::builder()
                    .header("Content-Type", "application/xml")
                    .status(context.status.as_status_code())
                    .body(Full::new(Bytes::from(body)))
            },
        };
        Ok(response?)
    }.boxed()
}

pub struct Response {
    status: HttpStatus,
    format: DataFormat,
}

impl Response {
    pub fn new(format: DataFormat, status: HttpStatus) -> Self {
        Self {
            status: status,
            format: format,
        }
    }

    pub fn set_status(mut self, status: HttpStatus) -> Self {
        self.status = status;
        self
    }
    
    pub fn set_format(mut self, format: DataFormat) -> Self {
        self.format = format;
        self
    }
    
}
