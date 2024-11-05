use actix_web::{HttpServer, App, web, Responder, middleware, HttpResponse, HttpRequest};
use env_logger::{Env};
use serde::{Deserialize, Serialize};
use log::info;


async fn index() -> impl Responder {
    "Hello world!"
}
#[derive(Deserialize, Serialize,Debug)]
struct User {
    username:String,
    email:String,
}

async fn receive( http_request: HttpRequest) ->impl Responder{
    info!("Received request :{}",http_request.path());
    let resp = custom_response::Response::new(200, "Ok".parse().unwrap(), "Hello world!");
    HttpResponse::Ok().json(resp)
}

async fn post_user(user: web::Json<User> ) -> impl Responder {

    let user = user.into_inner();
    info!("this user is {:?}",user);
    HttpResponse::Created().json(user)
}


#[tokio::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default().log_target("actix_web"))
            .service(
            web::scope("/api/v1")
                .route("/", web::get().to(index))
                .route("/post",web::post().to(post_user))
                .route("/info",web::get().to(receive))
        )
    })
        .bind("127.0.0.1:8080")?
        .workers(4)
        .run()
        .await
}

