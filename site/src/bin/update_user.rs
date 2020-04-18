extern crate diesel;
extern crate site;

use diesel::prelude::*;
use site::*;
use self::models::User;
use std::env::args;

fn main() {
    use self::schema::users::dsl::{users, activated};

    let id = args()
        .nth(1)
        .expect("update_user requires a user id")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = establish_connection();

    let user = diesel::update(users.find(id))
        .set(activated.eq(true))
        .get_result::<User>(&connection)
        .expect(&format!("Unable to find user {}", id));
    println!("\nActivated user {}\n", user.firstname);
}