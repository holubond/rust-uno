use actix_web::{web, Responder, put, HttpResponse};

use crate::handler::service::lb_connector::LoadBalancerConnector;

#[put("/restart")]
pub async fn lb_reconnect(
    lb_connector: web::Data<LoadBalancerConnector>,
) -> impl Responder {
    match lb_connector.connect().await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError()
    }    
}
