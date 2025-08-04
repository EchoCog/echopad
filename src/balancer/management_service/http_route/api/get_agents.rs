use actix_web::error::ErrorInternalServerError;
use actix_web::get;
use actix_web::web;
use actix_web::Error;
use actix_web::HttpResponse;

use crate::balancer::management_service::app_data::AppData;
use crate::produces_snapshot::ProducesSnapshot as _;

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(respond);
}

#[get("/api/v1/agents")]
async fn respond(
    app_data: web::Data<AppData>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(
        app_data.agent_controller_pool.make_snapshot().map_err(ErrorInternalServerError)?
    ))
}
