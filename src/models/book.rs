use diesel::prelude::Queryable;
use rocket::serde::Serialize;

#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Book {
    pub id: i32,
    pub name: String,
    pub author: String,
    pub year: i32,
}
