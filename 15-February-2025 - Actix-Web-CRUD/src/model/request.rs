use serde::Deserialize;

use super::User;

#[derive(Debug, Deserialize)]
pub struct ClientRegisterRequest {
    pub username: String,
    pub password: String,
    pub email: String,
}

impl TryFrom<ClientRegisterRequest> for User {
    type Error = String;

    fn try_from(value: ClientRegisterRequest) -> Result<Self, Self::Error> {
        if value.username.is_empty() {
            return Err("Username is needed to continue".into());
        }

        if value.password.is_empty() {
            return Err("Password is needed to continue".into());
        }
        // TODO: validate password strength ?

        if value.email.is_empty() {
            return Err("Email is needed to continue".into());
        }

        // TODO: validate email format ?

        Ok(User {
            username: value.username,
            password: value.password,
            email: value.email,
        })
    }
}

pub struct ClientLoginRequest {
    pub username: String,
    pub password: String,
}
