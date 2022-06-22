#[macro_use] extern crate rocket;
use rocket::serde::json::Json;

pub mod models;
pub mod schema;
pub mod services;

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