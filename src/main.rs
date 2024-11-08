use std::time::Duration;

use actix_web::{App,  HttpServer, middleware, Responder, web};
use actix_web::middleware::from_fn;
use env_logger::Env;
use log::info;

use custom::{custom_response::custom_response,custom_error::custom_error,custom_middleware::middleware as custom_middleware};

async fn greet(name: web::Path<String>) -> impl Responder {
    format!("hello {}!", name)
}


async fn custom_response() -> impl Responder {
    info!("data value");
    let resp =  custom_response::CustomResponse::new(vec![1,2,3], 200, "success".to_string());

    resp
}


async fn custom_error() ->impl Responder{

    custom_error::Error::new(1200400, "error".to_string())
}

async fn index() -> impl Responder {
    "Hello world!".to_string()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::try_init_from_env(Env::default().default_filter_or("info")).expect("TODO: panic message");

    HttpServer::new(move || {
        let get_route_group = web::scope("/v1");
        App::new()
            .wrap(middleware::Logger::default().log_target("http_log"))
            .wrap(from_fn(custom_middleware::print_request))
            .wrap(from_fn(custom_middleware::response_time))
            .wrap(from_fn(custom_middleware::get_header))
            .service(
                get_route_group.route("/hello/{name}", web::get().to(greet))
                    .route("/index", web::get().to(index))
                    .route("/response", web::post().to(custom_response))
                    .route("/error",web::post().to(custom_error))
            )
    })
        .workers(1)
        .keep_alive(Duration::from_secs(4))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}


