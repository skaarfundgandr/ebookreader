use serde::{Deserialize, Serialize};

use crate::data::models::users::Users;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDTO {
    pub username: String,
    pub email: String,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUserDTO {
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: Option<String>,
}

impl From<NewUserDTO> for UserDTO {
    fn from(user: NewUserDTO) -> Self {
        UserDTO {
            username: user.username.to_string(),
            email: user.email.to_string(),
            created_at: user.created_at.map(|s| s.to_string()),
        }
    }
}

impl From<Users> for UserDTO {
    fn from(user: Users) -> Self {
        UserDTO {
            username: user.username,
            email: user.email,
            created_at: user.created_at,
        }
    }
}
