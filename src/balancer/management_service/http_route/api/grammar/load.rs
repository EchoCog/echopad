use actix_web::{web, HttpResponse, Result as ActixResult};
use crate::balancer::management_service::app_data::AppData;
use crate::grammar_service::{LoadGrammarRequest, LoadGrammarResponse};
use log::debug;

pub async fn post(
    _app_data: web::Data<AppData>,
    request: web::Json<LoadGrammarRequest>,
) -> ActixResult<HttpResponse> {
    debug!("Grammar load request: {:?}", request);
    
    let response = LoadGrammarResponse {
        success: false,
        message: "Grammar service not yet integrated with balancer".to_string(),
    };
    
    Ok(HttpResponse::Ok().json(response))
}

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.route("/api/grammar/load", web::post().to(post));
}