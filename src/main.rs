#[macro_use]
extern crate rocket;

use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::{Build, Rocket};

use self::models::book::Book;
use self::schema::books::dsl::*;

mod database;
mod models;
mod schema;

#[get("/")]
fn index() -> Json<Vec<Book>> {
    let connection = &mut database::connection::establish_connection();
    books
        .load::<Book>(connection)
        .map(Json)
        .expect("Error loading books")
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![index])
}
