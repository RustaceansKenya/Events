use actix_web::{get, web::Data, HttpResponse};

use crate::model::AppState;

#[get("/")]
pub async fn home(data: Data<AppState>) -> HttpResponse {
    let html = format!(
        r#"
        <h1>Welcome to {}</h1>
        <p>Try the following endpoints:</p>
        <ul>
            <li><a href="/users">List all users</a></li>
            <li><a href="/users/1">Get user by username</a></li>
            <li><a href="/status">Health check</a></li>

            <li><a href="/register">Create a user</a></li>
        </ul>
        
    "#,
        data.app_name
    );

    HttpResponse::Ok().body(html)
}
