use rocket::{
    serde::json::{json, Json, Value},
    State,
};
use serde::Deserialize;

use crate::{
    auth::Auth,
    config::AppState,
    database::{self, Db},
    error::{Errors, FieldValidator},
};

#[derive(Deserialize)]
pub struct NewUser {
    user: NewUserData,
}

#[derive(Deserialize, Validate)]
struct NewUserData {
    #[validate(length(min = 4, max = 16))]
    username: Option<String>,
    #[validate(length(min = 8, max = 32))]
    password: Option<String>,
}

#[post("/users", format = "json", data = "<new_user>")]
pub async fn create_user(
    new_user: Json<NewUser>,
    db: Db,
    state: &State<AppState>,
) -> Result<Value, Errors> {
    let new_user = new_user.into_inner().user;

    let mut extractor = FieldValidator::validate(&new_user);
    let username = extractor.extract("username", new_user.username);
    let password = extractor.extract("password", new_user.password);

    extractor.check()?;
    let secret = state.secret.clone();

    db.run(move |conn| {
        database::users::create(conn, &username, &password)
            .map(|user| json!({ "user": user.to_auth(&secret) }))
            .map_err(|_| Errors::new(&[("username", "taken")]))
    })
    .await
}

#[derive(Deserialize)]
pub struct LoginUser {
    user: LoginUserData,
}

#[derive(Deserialize)]
struct LoginUserData {
    username: Option<String>,
    password: Option<String>,
}

#[post("/users/login", format = "json", data = "<user>")]
pub async fn login_user(
    user: Json<LoginUser>,
    db: Db,
    state: &State<AppState>,
) -> Result<Value, Errors> {
    let user = user.into_inner().user;

    let mut extractor = FieldValidator::default();
    let username = extractor.extract("username", user.username);
    let password = extractor.extract("password", user.password);
    extractor.check()?;

    let secret = state.secret.clone();
    db.run(move |conn| database::users::login(conn, &username, &password))
        .await
        .map(|user| json!({ "user": user.to_auth(&secret) }))
        .ok_or_else(|| Errors::new(&[("username_or_password", "is_invalid")]))
}

#[get("/user?<username>")]
pub async fn get_user(username: String, db: Db) -> Option<Value> {
    db.run(move |conn| database::users::find_by_username(conn, &username))
        .await
        .map(|user| json!({ "user": user.to_profile() }))
}

#[get("/user/whoami")]
pub async fn get_me(auth: Auth, db: Db, state: &State<AppState>) -> Option<Value> {
    let secret = state.secret.clone();
    db.run(move |conn| database::users::find(conn, auth.id))
        .await
        .map(|user| json!({ "user": user.to_auth(&secret) }))
}

#[derive(Deserialize)]
pub struct UpdateUser {
    user: database::users::UpdateUserData,
}

#[put("/user", format = "json", data = "<user>")]
pub async fn update_user(
    user: Json<UpdateUser>,
    auth: Auth,
    db: Db,
    state: &State<AppState>,
) -> Option<Value> {
    let secret = state.secret.clone();
    db.run(move |conn| database::users::update(conn, auth.id, &user.user))
        .await
        .map(|user| json!({ "user": user.to_auth(&secret) }))
}
