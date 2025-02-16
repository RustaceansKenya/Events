use std::sync::Mutex;

use super::user::User;

pub struct AppState {
    pub app_name: String,
    pub users: Mutex<Vec<User>>,
}
