#[macro_use] extern crate rocket;
use rocket::serde::json::Json;

extern crate diesel;
extern crate api;

use self::models::*;
use self::api::*;
use diesel::prelude::*;

#[get("/users")]
fn get_users() -> Json<Vec<User>> {
    use api::schema::users::dsl::*;

    let connection = establish_connection();
    let results = users
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading users");
    Json(results)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_users])
}