extern crate diesel;
extern crate site;

use diesel::prelude::*;
use site::*;
use std::env::args;

fn main() {
    use self::schema::users::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = establish_connection();
    let num_deleted = diesel::delete(users.filter(firstname.like(pattern)))
        .execute(&connection)
        .expect("Error deleting users");

    println!("\nDeleted {} users\n", num_deleted);
}