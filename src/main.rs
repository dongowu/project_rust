use std::time::Duration;

use actix_web::{
    App, Error, HttpRequest, HttpResponse, HttpServer, middleware, Responder, rt, web,
};
use actix_web::middleware::from_fn;
use actix_web::web::ServiceConfig;
use actix_ws::AggregatedMessage;
use env_logger::Env;
use futures_util::{StreamExt as _, TryStreamExt};
use log::info;
use shuttle_actix_web::ShuttleActixWeb;

use custom::{
    custom_error::custom_error, custom_middleware::middleware as custom_middleware,
    custom_response::custom_response,
};

async fn greet(name: web::Path<String>) -> impl Responder {
    format!("hello {}!", name)
}

async fn custom_response() -> impl Responder {
    info!("data value");
    let resp = custom_response::CustomResponse::new(vec![1, 2, 3], 200, "success".to_string());

    resp
}

async fn echo(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

    let mut stream = stream
        .aggregate_continuations()
        .max_continuation_size(2_usize.pow(20));

    rt::spawn(async move {
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(AggregatedMessage::Text(text)) => {
                    session.text(text).await.unwrap();
                }
                Ok(AggregatedMessage::Binary(bin)) => {
                    session.binary(bin).await.unwrap();
                }
                Ok(AggregatedMessage::Ping(msg)) => {
                    session.pong(&msg).await.unwrap();
                }
                _ => {}
            }
        }
    });
    Ok(res)
}

struct AppState {
    app_name: String,
}

async fn index_state(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {}", app_name)
}

async fn custom_error() -> impl Responder {
    custom_error::Error::new(1200400, "error".to_string())
}

async fn index() -> impl Responder {
    "Hello world!".to_string()
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    env_logger::try_init_from_env(Env::default().default_filter_or("info"))
        .expect("TODO: panic message");


    let app = move |cfg: &mut ServiceConfig| {
        let get_route_group = web::scope("/v1")
            .route("/hello/{name}", web::get().to(greet))
            .route("/index", web::get().to(index))
            .route("/response", web::post().to(custom_response))
            .route("/error", web::post().to(custom_error))
            .route("/index1", web::get().to(index_state))
        .wrap(middleware::Logger::default().log_target("http_log"))
        .wrap(from_fn(custom_middleware::print_request))
        .wrap(from_fn(custom_middleware::response_time))
        .wrap(from_fn(custom_middleware::get_header));
        // .wrap(from_fn(custom_middleware::get_state))


        cfg.app_data(
            web::Data::new(AppState {
                app_name: "actix-web".to_string(),
            })
                .clone(),
        ).service(get_route_group)


    };


    Ok(app.into())
}
