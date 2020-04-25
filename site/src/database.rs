extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use site::models::{NewUser, User};
use site::schema::users::dsl::users;

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn add_user(first: String, last: String) {
    let new_user = NewUser {
        firstname: &first,
        lastname: &last,
    };

    let connection = establish_connection();
    diesel::insert_into(users)
        .values(&new_user)
        .get_result::<User>(&connection)
        .expect("Error saving new user");
}