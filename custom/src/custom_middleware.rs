
pub  mod middleware{
    use std::collections::HashMap;
    use actix_web::body::MessageBody;
    use actix_web::dev::{ServiceRequest, ServiceResponse};
    use actix_web::{Error, web};
    use actix_web::middleware::Next;
    use actix_web::web::Query;
    use log::info;
    use crate::custom_middleware::AppState;

    pub async fn print_request(
        string_body: String,
        query: Query<HashMap<String, String>>,
        req: ServiceRequest,
        next: Next<impl MessageBody + 'static>,
    ) -> Result<ServiceResponse<impl MessageBody>, Error> {
        info!("request path:{}",req.path());
        info!("request body:{}",string_body);
        info!("query:{:?}",query);


        Ok(next.call(req).await?)
    }

    pub async fn response_time(
        req: ServiceRequest,
        next: Next<impl MessageBody + 'static>,
    ) -> Result<ServiceResponse<impl MessageBody>, Error> {
        let start = std::time::Instant::now();
        let res = next.call(req).await?;
        let duration = start.elapsed();
        info!("response time:{}",duration.as_millis());
        Ok(res)
    }


    pub async fn get_header(
        req: ServiceRequest,
        next: Next<impl MessageBody + 'static>,
    ) -> Result<ServiceResponse<impl MessageBody>, Error> {
        let header = req.headers();
        // header.get("user-agent");
        info!("header:{:?}",header);
        Ok(next.call(req).await?)
    }

    pub async fn get_state(state :web::Data<AppState>, req :ServiceRequest,next: Next<impl MessageBody +'static>)->Result<ServiceResponse<impl MessageBody>,Error>{

        info!("state,{:?}",state.app_name);
        Ok(next.call(req).await?)
    }
}


pub struct AppState{
    app_name:String,
}