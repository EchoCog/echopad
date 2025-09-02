use actix_web::{web, HttpResponse, Result as ActixResult};
use crate::balancer::management_service::app_data::AppData;
use crate::grammar_service::{ParseRequest, ParseResponse};
use log::debug;

pub async fn post(
    _app_data: web::Data<AppData>,
    request: web::Json<ParseRequest>,
) -> ActixResult<HttpResponse> {
    debug!("Grammar parse request: {:?}", request);
    
    // In a full implementation, we'd get the grammar service from app_data
    // For now, create a simple response
    let response = ParseResponse {
        success: false,
        parse_tree: None,
        error: Some("Grammar service not yet integrated with balancer".to_string()),
    };
    
    Ok(HttpResponse::Ok().json(response))
}

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.route("/api/grammar/parse", web::post().to(post));
}