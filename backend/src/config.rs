use std::{collections::HashMap, env, net::{IpAddr, Ipv4Addr}};

use rocket::{
    data::{Limits, ToByteUnit},
    fairing::AdHoc,
    figment::Figment,
    Config,
};

use crate::filemanager::FileManager;

const SECRET: &str = "8Xui8SN4mI+7egV/9dlfYYLGQJeEx4+DwmSQLwDVXJg=";

pub const DATE_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";

pub const TOKEN_PREFIX: &str = "Token ";

pub struct AppState {
    pub secret: Vec<u8>,
    pub filemanager: FileManager,
}

impl AppState {
    pub fn manage() -> AdHoc {
        AdHoc::on_ignite("Manage config", |rocket| async move {
            let secret = env::var("SECRET_KEY").unwrap_or_else(|err| {
                if cfg!(debug_assertions) {
                    SECRET.to_string()
                } else {
                    panic!("No SECRET_KEY environment variable found: {:?}", err)
                }
            });

            let filemanager = FileManager::new().await;

            rocket.manage(AppState {
                secret: secret.into_bytes(),
                filemanager,
            })
        })
    }
}

pub fn from_env() -> Figment {
    let address = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .expect("PORT environment variable should parse to an integer");

    let upload_limit = env::var("UPLOAD_LIMIT")
        .unwrap_or_else(|_| "10".to_string())
        .parse::<u64>()
        .expect("UPLOAD_LIMIT environment variable should parse to an integer")
        .megabytes();

    let limits = Limits::default()
        .limit("data-form", upload_limit)
        .limit("bytes", upload_limit);

    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();
    let database_url =
        env::var("DATABASE_URL").expect("No DATABASE_URL environment variable found");
    database_config.insert("url", database_url);
    databases.insert("diesel_postgres_pool", database_config);

    Config::figment()
        .merge(("port", port))
        .merge(("limits", limits))
        .merge(("databases", databases))
        .merge(("address", address))
}
