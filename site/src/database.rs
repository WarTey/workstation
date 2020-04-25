use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use crate::models::{NewUser, User};
use crate::schema::users::dsl::{users, firstname, activated};

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

pub fn delete_user(first: String) {
    let connection = establish_connection();
    diesel::delete(users.filter(firstname.like(first)))
        .execute(&connection)
        .expect("Error deleting users");
}

pub fn update_user(id: i32) {
    let connection = establish_connection();
    diesel::update(users.find(id))
        .set(activated.eq(true))
        .get_result::<User>(&connection)
        .expect(&format!("Unable to find user {}", id));
}

pub fn show_user() -> Vec<User> {
    let connection = establish_connection();
    users.filter(activated.eq(true))
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading users")
}
