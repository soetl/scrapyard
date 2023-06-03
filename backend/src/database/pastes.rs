use crate::{
    models::{
        paste::{Paste, PasteJson},
        user::User,
    },
    schema::{pastes, users}, error::Errors,
};

use diesel::prelude::*;
use uuid::Uuid;

const DEFAULT_LIMIT: i64 = 20;

#[derive(Insertable)]
#[diesel(table_name = pastes)]
struct NewPaste<'a> {
    owner: i32,
    filename: &'a str,
    link: &'a str,
    type_: &'a str,
    mime: &'a str,
}

pub fn create(
    conn: &mut PgConnection,
    filename: &str,
    owner: i32,
    type_: &str,
    mime: &str,
) -> PasteJson {
    let binding = Uuid::new_v4().hyphenated().to_string();
    let new_paste = NewPaste {
        filename,
        link: binding.as_str(),
        owner,
        type_,
        mime,
    };

    let owner = users::table
        .find(owner)
        .get_result::<User>(conn)
        .expect("Failed to find owner");

    diesel::insert_into(pastes::table)
        .values(new_paste)
        .get_result::<Paste>(conn)
        .expect("Failed to create paste")
        .to_json(owner)
}

#[derive(FromForm, Default, Debug)]
pub struct FindPastes {
    pub owner: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub fn find(conn: &mut PgConnection, params: &FindPastes) -> (Vec<PasteJson>, i64) {
    let mut query = pastes::table
        .inner_join(users::table)
        .select((pastes::all_columns, users::all_columns))
        .into_boxed();

    if let Some(owner) = &params.owner {
        query = query.filter(users::username.eq(owner));
    };

    query
        .order(pastes::created_at.desc())
        .limit(params.limit.unwrap_or(DEFAULT_LIMIT))
        .offset(params.offset.unwrap_or(0))
        .load::<(Paste, User)>(conn)
        .map(|res| {
            let count = res.len() as i64;
            (
                res.into_iter()
                    .map(|(paste, owner)| paste.to_json(owner))
                    .collect(),
                count,
            )
        })
        .expect("Failed load pastes")
}

pub fn find_one(conn: &mut PgConnection, link: &str) -> Option<PasteJson> {
    let paste = pastes::table
        .filter(pastes::link.eq(link))
        .first::<Paste>(conn)
        .map_err(|e| println!("articles::find_one: {}", e))
        .ok()?;

    Some(populate(conn, paste))
}

pub fn delete(conn: &mut PgConnection, link: &str, user_id: i32) -> Result<(), Errors> {
    let result =
        diesel::delete(pastes::table.filter(pastes::link.eq(link).and(pastes::owner.eq(user_id))))
            .execute(conn);

    if let Err(e) = result {
        println!("pastes::delete: {}", e);
        return Err(Errors::new(&[("paste", "db_delete_failed")]));
    }

    Ok(())
}

fn populate(conn: &mut PgConnection, paste: Paste) -> PasteJson {
    let owner = users::table
        .find(paste.owner)
        .get_result::<User>(conn)
        .expect("Failed to find owner");

    paste.to_json(owner)
}
