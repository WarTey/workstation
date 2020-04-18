extern crate diesel;
extern crate site;

use diesel::prelude::*;
use site::*;
use self::models::*;

fn main() {
    use self::schema::users::dsl::*;

    let connection = establish_connection();
    let results = users
        .filter(activated.eq(true))
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading users");

    println!("\nDisplaying {} user(s):", results.len());
    for user in results {
        println!("{}. {} {}", user.id, user.firstname, user.lastname);
    }
    println!("");
}