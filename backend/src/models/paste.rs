use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::config::DATE_FORMAT;
use crate::models::user::User;

#[derive(Queryable)]
pub struct Paste {
    pub id: i32,
    pub owner: i32,
    pub filename: String,
    pub link: String,
    pub r#type: String,
    pub password_protected: bool,
    pub password_hash: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PasteJson {
    pub id: String,
    pub owner: User,
    pub filename: String,
    pub link: String,
    pub r#type: String,
    pub password_protected: bool,
    pub created_at: String,
}

impl Paste {
    pub fn to_json(&self, owner: User) -> PasteJson {
        PasteJson {
            id: self.id.to_string(),
            owner,
            filename: self.filename.to_owned(),
            link: self.link.to_owned(),
            r#type: self.r#type.to_owned(),
            password_protected: self.password_protected,
            created_at: self.created_at.format(DATE_FORMAT).to_string(),
        }
    }
}
