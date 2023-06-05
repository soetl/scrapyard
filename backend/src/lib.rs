#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket::serde::json::{json, Value};

#[macro_use]
extern crate rocket_sync_db_pools;

extern crate rocket_cors;
use rocket_cors::{Cors, CorsOptions};

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate validator_derive;

mod auth;
mod config;
mod database;
mod error;
mod filemanager;
mod models;
mod routes;
mod schema;

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found"
    })
}

fn cors_fairing() -> Cors {
    CorsOptions::default()
        .to_cors()
        .expect("Cors fairing cannot be created")
}

#[launch]
pub fn rocket() -> _ {
    //dotenv::dotenv().ok();
    rocket::custom(config::from_env())
        .mount(
            "/api/v1",
            routes![
                routes::users::create_user,
                routes::users::login_user,
                routes::users::get_user,
                routes::users::get_me,
                routes::users::update_user,
                routes::pastes::upload_paste,
                routes::pastes::download_paste,
                routes::pastes::get_pastes,
                routes::pastes::get_paste,
                routes::pastes::delete_paste,
            ],
        )
        .attach(database::Db::fairing())
        .attach(cors_fairing())
        .attach(config::AppState::manage())
        .register("/", catchers![not_found])
}
