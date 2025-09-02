use actix_web::{web, HttpResponse, Result as ActixResult};
use crate::balancer::management_service::app_data::AppData;
use crate::grammar_service::{GenerateCodeRequest, GenerateCodeResponse};
use log::debug;

pub async fn post(
    _app_data: web::Data<AppData>,
    request: web::Json<GenerateCodeRequest>,
) -> ActixResult<HttpResponse> {
    debug!("Grammar generate request: {:?}", request);
    
    let response = GenerateCodeResponse {
        success: false,
        code: None,
        error: Some("Grammar service not yet integrated with balancer".to_string()),
    };
    
    Ok(HttpResponse::Ok().json(response))
}

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.route("/api/grammar/generate", web::post().to(post));
}