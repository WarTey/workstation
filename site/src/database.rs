use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use crate::models::{NewUser, User};
use crate::schema::users::dsl::{users, firstname, email, token, activated};

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn add_user(first: String, last: String, mail: String) {
    let new_user = NewUser {
        firstname: &first,
        lastname: &last,
        email: &mail
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

pub fn update_user_activation(mail: String, status: bool) {
    let connection = establish_connection();
    diesel::update(users.filter(email.eq(mail)))
        .set(activated.eq(status))
        .get_result::<User>(&connection)
        .expect("Error updating user...");
}

pub fn check_link(link: String) -> bool {
    let connection = establish_connection();
    let results = users.filter(token.eq(link))
        .limit(1)
        .load::<User>(&connection)
        .expect("Error loading users");

    return if results.len() == 1 && results[0].email.len() > 0 {
        true
    } else {
        false
    }
}

pub fn show_user() -> Vec<User> {
    let connection = establish_connection();
    users.filter(activated.eq(true))
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading users")
}
