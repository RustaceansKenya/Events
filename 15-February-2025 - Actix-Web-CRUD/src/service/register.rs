use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Result,
};
use serde_json::json;

use crate::{
    error::Error,
    model::{AppState, ClientRegisterRequest},
};

#[post("/users")]
async fn register(
    data: Data<AppState>,
    payload: Json<ClientRegisterRequest>,
) -> Result<HttpResponse> {
    log::info!("Register: {:?}", payload);

    // Step 1: Validate the request & create a user with TryFrom
    let user = payload
        .into_inner()
        .try_into()
        .map_err(Error::ValidationError)?;

    // add user to the state
    let mut state = data.users.lock().unwrap();
    state.push(user);

    Ok(HttpResponse::Ok().json(json!({
        "msg": "User registered successfully",
        "success": true
    })))
}
