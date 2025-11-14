use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32, // Subject (user_id)
    pub exp: usize, // Expiration time
}

pub struct Tokenizer {
    pub secret_key: String,
    pub expiration_duration: i64,
}

impl Tokenizer {
    pub async fn get_instance() -> &'static Tokenizer {
        &TOKENIZER
    }

    pub fn generate_token(&self, user_id: i32) -> Result<String, jsonwebtoken::errors::Error> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::seconds(self.expiration_duration))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: user_id,
            exp: expiration,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret_key.as_ref()),
        )
    }

    pub fn decode_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let mut validation = Validation::default();
        validation.validate_exp = true;
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret_key.as_ref()),
            &validation,
        )
        .map(|data| data.claims)
    }

    pub fn refresh_token(&self, _token: &str) -> String {
        // TODO: Implement JWT token refresh
        unimplemented!()
    }
}

static TOKENIZER: Lazy<Tokenizer> = Lazy::new(|| {
    // TODO: Move secret_key to an environment variable
    Tokenizer {
        secret_key: "a_very_secret_key_that_should_be_in_env".to_string(),
        expiration_duration: 3600, // 1 hour
    }
});
