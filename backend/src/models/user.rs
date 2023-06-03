use chrono::{Duration, Utc};
use serde::Serialize;

use crate::auth::Auth;

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub username: String,
    pub image: Option<String>,
    #[serde(skip_serializing)]
    pub password_hash: String,
}

#[derive(Serialize)]
pub struct UserAuth<'a> {
    username: &'a str,
    image: Option<&'a str>,
    token: String,
}

#[derive(Serialize)]
pub struct UserProfile {
    pub username: String,
    pub image: Option<String>,
}

impl User {
    pub fn to_auth(&self, secret: &[u8]) -> UserAuth {
        let exp = Utc::now() + Duration::days(34);
        let token = Auth {
            id: self.id,
            username: self.username.clone(),
            exp: exp.timestamp(),
        }
        .token(secret);

        UserAuth {
            username: &self.username,
            image: self.image.as_deref(),
            token,
        }
    }

    pub fn to_profile(&self) -> UserProfile {
        UserProfile {
            username: self.username.to_owned(),
            image: self.image.to_owned(),
        }
    }
}
