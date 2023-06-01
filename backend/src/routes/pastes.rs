use rocket::{
    form::{Form, Strict},
    serde::json::{json, Value},
    State, http::Status,
};
use validator::ValidationError;

use crate::{
    auth::Auth,
    config::AppState,
    database::{self, Db, pastes::FindPastes},
    error::{Errors, FieldValidator},
};

#[derive(FromForm, Validate, Debug)]
pub struct NewPaste<'r> {
    #[validate(length(min = 1, max = 32))]
    pub filename: String,
    #[validate(custom = "validate_type")]
    pub r#type: String,
    pub content: &'r [u8],
    pub password_protected: bool,
    pub password: String,
}

#[post("/pastes", data = "<new_paste>")]
pub async fn upload_paste(
    auth: Auth,
    new_paste: Form<Strict<NewPaste<'_>>>,
    db: Db,
    state: &State<AppState>,
) -> Result<Value, Errors> {
    let content = new_paste.content.to_vec();
    let new_paste: NewPaste = new_paste.into_inner().into_inner();
    println!("{:?}", new_paste);

    let mut extractor = FieldValidator::validate(&new_paste);
    let filename = extractor.extract("filename", Some(new_paste.filename));
    let type_ = extractor.extract("type", Some(new_paste.r#type));
    let password_protected = new_paste.password_protected;
    let password = new_paste.password;
    extractor.check()?;

    let paste = db
        .run(move |conn| {
            database::pastes::create(
                conn,
                &filename,
                auth.id,
                &type_,
                password_protected,
                Some(&password),
            )
        })
        .await;

    match state.filemanager.upload(format!("{}/{}", paste.link, paste.filename).as_str(), &content).await {
        Ok(_) => (),
        Err(_) => {
            return Err(Errors::new(&[("file", "file upload failed")]));
        }
    }

    Ok(json!({ "paste": paste }))
}

#[get("/pastes/<link>/<filename>")]
pub async fn download_paste(link: &str, filename: &str, state: &State<AppState>)  -> Result<Vec<u8>, Status> {
    let file = state.filemanager.download(format!("{}/{}", link, filename).as_str()).await;

    match file {
        Ok(file) => Ok(file.bytes().to_vec()),
        Err(_) => Err(Status::NotFound),
    }
}

#[get("/pastes?<params>")]
pub async fn get_pastes(params: FindPastes, db: Db) -> Result<Value, Errors> {
    let pastes = db
        .run(move |conn| database::pastes::find(conn, &params))
        .await;

    Ok(json!({ "pastes": pastes }))
}

#[get("/pastes/<link>")]
pub async fn get_paste(link: String, db: Db) -> Result<Value, Errors> {
    let paste = db
        .run(move |conn| database::pastes::find_one(conn, &link))
        .await;

    Ok(json!({ "paste": paste }))
}

#[delete("/pastes/<link>")]
pub async fn delete_paste(link: String, auth: Auth, db: Db, state: &State<AppState>) -> Result<Value, Errors> {
    let link_copy = link.clone();

    let this_paste = db
        .run(move |conn| database::pastes::find_one(conn, &link_copy))
        .await;

    if let Some(paste) = this_paste {
        match state.filemanager.delete(format!("{}/{}", paste.link, paste.filename).as_str()).await {
            Ok(_) => (),
            Err(_) => {
                return Err(Errors::new(&[("file", "file deletion failed")]));
            }
        }
    } else {
        return Err(Errors::new(&[("paste", "paste not found")]));
    }

    let paste = db
        .run(move |conn| database::pastes::delete(conn, &link, auth.id))
        .await;

    Ok(json!({ "paste": paste }))
}

fn validate_type(type_: &str) -> Result<(), ValidationError> {
    match type_ {
        "text" | "file" => Ok(()),
        _ => Err(ValidationError::new("type must be either text or file")),
    }
}
