use rocket::{serde::json::{json, Value}, http::Status};

use crate::database::{Db, self};

#[get("/profiles/<username>")]
pub async fn get_profile(username: String, db: Db) -> Result<Value, Status> {
    let user = db
        .run(move |conn| database::users::find_by_username(conn, &username))
        .await
        .ok_or(Status::NotFound)?;

    Ok(json!({ "user": user.to_profile() }))
}