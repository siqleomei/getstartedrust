extern crate diesel;
extern crate getstartedrust;

use self::models::*;
use self::getstartedrust::*;
use diesel::prelude::*;

fn main() {
    use getstartedrust::schema::users::dsl::*;

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
