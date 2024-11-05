use serde::{Deserialize, Serialize};
use actix_web::{HttpRequest, HttpResponse, http::header::ContentType, Responder};

#[derive(Serialize, Deserialize)]
pub struct Response<T> {
    pub status_code: i32,
    pub msg: String,
    pub data: T,
}

impl<T> Response<T> {
    pub fn new(status_code: i32, msg: String, data: T) -> Self {
        Self {
            status_code,
            msg,
            data,
        }
    }
}

impl<T> Responder for Response<T>
where
    T: Serialize,  // 确保 T 类型可序列化
{
    type Body = String;  // 响应体类型是 String，因为我们将它转化为 JSON 字符串

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        // 直接使用 .json() 方法来序列化响应体
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(self)  // `self` 会自动序列化
    }
}
