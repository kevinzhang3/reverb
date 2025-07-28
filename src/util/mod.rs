pub enum DataFormat {
    JSON(String),
    XML(String),
}

pub enum HttpStatus {
    Ok = 200,
    NotFound = 404,
}

impl HttpStatus {
    pub fn as_u16(self) -> u16 {
        self as u16
    }

    pub fn as_status_code(self) -> hyper::StatusCode {
        hyper::StatusCode::from_u16(self.as_u16())
            .unwrap_or(hyper::StatusCode::INTERNAL_SERVER_ERROR)
    }
}
