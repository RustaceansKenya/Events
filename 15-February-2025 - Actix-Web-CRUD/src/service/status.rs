use actix_web::{get, HttpResponse};
use serde_json::json;

#[get("/status")]
pub async fn status() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "success": true,
        "msg": "Server is running"
    }))
}
