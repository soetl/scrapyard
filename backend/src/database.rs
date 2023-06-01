pub mod users;
pub mod pastes;

#[database("diesel_postgres_pool")]
pub struct Db(diesel::PgConnection);