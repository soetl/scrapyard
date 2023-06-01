use jsonwebtoken as jwt;

use jwt::{EncodingKey, DecodingKey, Validation, Algorithm};
use rocket::{request::{FromRequest, self}, Request, outcome::Outcome, http::Status};
use serde::{Deserialize, Serialize};

use crate::config::{self, AppState};

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
    pub exp: i64,
    pub id: i32,
    pub username: String,
}

impl Auth {
    pub fn token(&self, secret: &[u8]) -> String {
        let encoding_key = EncodingKey::from_base64_secret(std::str::from_utf8(secret).unwrap());
        jwt::encode(&jwt::Header::default(), self, &encoding_key.unwrap()).expect("jwt")
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let state = req.rocket().state::<AppState>().unwrap();
        if let Some(auth) = extract_auth_from_request(req, &state.secret) {
            Outcome::Success(auth)
        } else {
            Outcome::Failure((Status::Forbidden, ()))
        }
    } 
}

fn extract_auth_from_request(request: &Request, secret: &[u8]) -> Option<Auth> {
    request
        .headers()
        .get_one("authorization")
        .and_then(extract_token_from_header)
        .and_then(|token| decode_token(token, secret))
}

fn extract_token_from_header(header: &str) -> Option<&str> {
    if header.starts_with(config::TOKEN_PREFIX) {
        Some(&header[config::TOKEN_PREFIX.len()..])
    } else {
        None
    }
}

fn decode_token(token: &str, secret: &[u8]) -> Option<Auth> {
    let decoding_key = DecodingKey::from_base64_secret(std::str::from_utf8(secret).unwrap());

    jwt::decode(
        token,
        &decoding_key.unwrap(),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|err| {
        eprintln!("Auth decode error: {:?}", err);
    })
    .ok()
    .map(|token_data| token_data.claims)
}