extern crate diesel;
extern crate api;

use self::models::*;
use self::api::*;
use diesel::prelude::*;

fn main() {
    use api::schema::users::dsl::*;

    let connection = establish_connection();
    let results = users
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading users");
    println!("Displaying {} users", results.len());
    for user in results {
        println!("{} mora no endereço {}", user.name, user.address);
        println!("----------\n");
    }
}
