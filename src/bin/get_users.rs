#[macro_use] extern crate rocket;
use getstartedrust::{models::User, establish_connection};
use rocket::serde::json::Json;

extern crate getstartedrust;
use diesel::prelude::*;

#[get("/users")]
fn get_users() -> Json<Vec<User>> {
    use getstartedrust::schema::users::dsl::*;

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