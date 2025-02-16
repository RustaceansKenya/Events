use actix_web::{
    delete, get,
    web::{Data, Path},
    HttpResponse,
};
use serde_json::json;

use crate::{error::Error, model::AppState};

#[get("/users")]
pub async fn users(data: Data<AppState>) -> Result<HttpResponse, Error> {
    let users = &data
        .users
        .lock()
        .map_err(|_| Error::InternalError("Lock Error".to_string()))?;

    Ok(HttpResponse::Ok().json({
        json!({
            "data": users.to_vec(),
            "success": true
        })
    }))
}

#[get("/users/{username}")]
pub async fn user_by_username(
    data: Data<AppState>,
    username: Path<String>,
) -> Result<HttpResponse, Error> {
    let users_ = &data
        .users
        .lock()
        .map_err(|_| Error::InternalError("Lock Error".to_string()))?;

    log::info!("Finding user by username: {}", username);

    let user = users_
        .iter()
        .find(|u| u.username == username.to_string())
        .ok_or(Error::NotFound)?;

    Ok(HttpResponse::Ok().json({
        json!({
            "data": user,
            "success": true
        })
    }))
}

#[delete("/users/{username}")]
pub async fn delete_user_by_username(
    data: Data<AppState>,
    username: Path<String>,
) -> Result<HttpResponse, Error> {
    let mut users_ = data
        .users
        .lock()
        .map_err(|_| Error::InternalError("Lock Error".to_string()))?;

    log::info!("Deleting user by username: {}", username);

    let user = users_
        .iter()
        .position(|u| u.username == username.to_string())
        .ok_or(Error::NotFound)?;

    users_.remove(user);

    Ok(HttpResponse::Ok().json({
        json!({
            "msg": "User deleted successfully",
            "success": true
        })
    }))
}
