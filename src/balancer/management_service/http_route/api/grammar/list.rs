use actix_web::{web, HttpResponse, Result as ActixResult};
use crate::balancer::management_service::app_data::AppData;
use log::debug;

pub async fn get(
    _app_data: web::Data<AppData>,
) -> ActixResult<HttpResponse> {
    debug!("Grammar list request");
    
    // Return empty list for now
    let grammars: Vec<String> = vec![
        "ArithmeticGrammar".to_string(),
        "JsonGrammar".to_string(), 
        "ZPlusPlus".to_string(),
    ];
    
    Ok(HttpResponse::Ok().json(grammars))
}

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.route("/api/grammar/list", web::get().to(get));
}