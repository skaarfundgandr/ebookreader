use serde::{Deserialize, Serialize};

use crate::data::models::users::Users;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDTO {
    pub username: String,
    pub email: String,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUserDTO<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password_hash: &'a str,
    pub created_at: Option<&'a str>,
}

impl NewUserDTO<'_> {
    pub fn to_user_dto(&self) -> UserDTO {
        UserDTO {
            username: self.username.to_string(),
            email: self.email.to_string(),
            created_at: self.created_at.map(|s| s.to_string()),
        }
    }
}

impl From<NewUserDTO<'_>> for UserDTO {
    fn from(user: NewUserDTO<'_>) -> Self {
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
