use chrono::prelude::*;
use jsonwebtoken::{EncodingKey, Header};
use serde::Serialize;

use crate::models::LoginInfoDTO;

static ONE_WEEK: i64 = 60 * 60 * 24 * 7;
pub static KEY: [u8; 16] = *include_bytes!("../../secret.key");

#[derive(Serialize)]
pub struct TokenPayload {
    pub iat: i64,
    pub exp: i64,
    pub user_id: String,
    pub user_email: String,
}

impl TokenPayload {
    pub fn generate_token(login_info_dto: &LoginInfoDTO) -> String {
        let now = Utc::now().timestamp_nanos() / 1_000_000_000;
        let payload = TokenPayload {
            iat: now,
            exp: now + ONE_WEEK,
            user_id: login_info_dto.id.clone(),
            user_email: login_info_dto.email.clone(),
        };

        jsonwebtoken::encode(
            &Header::default(),
            &payload,
            &EncodingKey::from_secret(&KEY),
        )
        .unwrap()
    }
}
