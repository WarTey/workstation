extern crate diesel;
extern crate site;

use diesel::prelude::*;
use site::*;
use self::models::{NewUser, User};
use std::io::stdin;

fn main() {
    use self::schema::users::dsl::*;

    let mut first = String::new();
    let mut last = String::new();

    println!("\nWhat would you like your firstname to be?");
    stdin().read_line(&mut first).unwrap();
    let first = first.trim_end();

    println!("What would you like your lastname to be?");
    stdin().read_line(&mut last).unwrap();
    let last = last.trim_end();

    let new_user = NewUser {
        firstname: first,
        lastname: last,
    };

    let connection = establish_connection();
    let user = diesel::insert_into(users)
        .values(&new_user)
        .get_result::<User>(&connection)
        .expect("Error saving new user");
    println!("\nSaved user {} with id {}\n", first, user.id);
}
