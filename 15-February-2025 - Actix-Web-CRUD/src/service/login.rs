use actix_web::{post, web::Data};

use crate::model::AppState;

#[post("/login")]
pub async fn login(data: Data<AppState>) -> String {
    let app_name = &data.app_name; // get app_name from state
    format!("login, {}!", app_name)
}
