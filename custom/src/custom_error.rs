pub mod custom_error {
    use std::fmt::Formatter;

    use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
    use serde::Serialize;
    use serde_json;
    #[derive(Serialize, Debug)]
    pub struct Error {
        pub code: u32,
        pub msg: String,
    }


    impl Error {
        pub fn new(code: u32, msg: String) -> Self {
            Error { code, msg }
        }
    }


    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Error {}: {}", self.code, self.msg)
        }
    }

    impl ResponseError for Error {

        fn error_response(&self) -> HttpResponse {
            let error_json = serde_json::json!(self.to_string());
            HttpResponse::Ok().json(error_json)
        }
    }


    impl Responder for Error {
        type Body = actix_web::body::BoxBody;

        fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {

            HttpResponse::Ok().json(self)
        }
    }

}
