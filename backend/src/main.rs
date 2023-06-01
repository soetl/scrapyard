use backend::rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _ = rocket().launch().await;
    Ok(())
}