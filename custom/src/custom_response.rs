pub mod custom_response {
    use actix_web::{HttpRequest, HttpResponse, Responder};
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize)]
    pub struct CustomResponse<T> {
        data: T,
        status: i16,
        msg: String,
    }

    impl<T> CustomResponse<T> {
        pub fn new(data: T, status: i16, msg: String) -> Self {
            CustomResponse { data, status, msg }
        }
    }
    //
    impl<T> Responder for CustomResponse<T>
    where
        T: Serialize, // 这里确保 T 可序列化
    {
        type Body = actix_web::body::BoxBody;

        fn respond_to(self, _req: &HttpRequest) -> HttpResponse {
            // 这里我们可以直接使用 .json() 方法来序列化整个 CustomResponse 对象
            HttpResponse::Ok().json(self) // `self` 会自动序列化成 JSON
        }
    }
}
