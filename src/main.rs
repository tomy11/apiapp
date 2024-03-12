#[macro_use] extern crate rocket;
use rocket_db_pools::{Database, Connection};
use rocket_db_pools::sqlx::{self, Row};

#[derive(Database)]
#[database("sqlite_logs")]
struct Logs(sqlx::SqlitePool);

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/context")]
async fn read(mut db: Connection<Logs>) -> Option<String> {
   sqlx::query("SELECT * FROM context")
       .fetch_one(&mut **db).await
       .and_then(|r| Ok(r.try_get(0)?))
       .ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, read])
}
