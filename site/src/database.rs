use diesel::prelude::*;
use rand::prelude::*;
use dotenv::dotenv;
use std::env;

use crate::models::{NewUser, User};
use crate::schema::users::dsl::{users, email, token, activated, pass_strength, crack_time};

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn add_user(first: String, last: String, mail: String) {
    let new_user = NewUser {
        firstname: &first,
        lastname: &last,
        email: &mail,
        pass_strength: "undefined".to_string(),
        crack_time: "undefined".to_string(),
    };

    let connection = establish_connection();
    diesel::insert_into(users)
        .values(&new_user)
        .load::<User>(&connection)
        .unwrap();
}

/*pub fn delete_user(first: String) {
    let connection = establish_connection();
    diesel::delete(users.filter(firstname.like(first)))
        .execute(&connection)
        .expect("Error deleting users");
}*/

pub fn update_user_link(mail: String) {
    let connection = establish_connection();
    let new_token: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(32)
        .collect();
    diesel::update(users.filter(email.eq(mail)))
        .set((activated.eq(false), token.eq(format!("{:x}", md5::compute(new_token)))))
        .execute(&connection)
        .unwrap();
}

pub fn check_status(link: String) -> bool {
    let connection = establish_connection();
    users.filter(token.eq(link).and(activated.eq(true)))
        .limit(1)
        .load::<User>(&connection)
        .unwrap()
        .len() == 1
}

pub fn check_email(mail: String) -> bool {
    let connection = establish_connection();
    users.filter(email.eq(mail))
        .limit(1)
        .load::<User>(&connection)
        .unwrap()
        .len() == 1
}

pub fn check_link(link: String) -> bool {
    let connection = establish_connection();
    users.filter(token.eq(link))
        .limit(1)
        .load::<User>(&connection)
        .unwrap()
        .len() == 1
}

pub fn get_email_from_link(link: String) -> String {
    let connection = establish_connection();
    users.filter(token.eq(link))
        .limit(1)
        .load::<User>(&connection)
        .unwrap()[0]
        .email.to_string()
}

pub fn get_link_from_email(mail: String) -> String {
    let connection = establish_connection();
    users.filter(email.eq(mail))
        .limit(1)
        .load::<User>(&connection)
        .unwrap()[0]
        .token.to_string()
}

pub fn update_user_password(mail: String, password: String) {
    let connection = establish_connection();
    let estimate = zxcvbn::zxcvbn(&password, &[]).unwrap();
    diesel::update(users.filter(email.eq(mail)))
        .set((pass_strength.eq(format!("{}", estimate.score())), crack_time.eq(format!("{}", estimate.crack_times().online_no_throttling_10_per_second()))))
        .execute(&connection)
        .unwrap();
}

pub fn create_user_password(mail: String, password: String) {
    let connection = establish_connection();
    let estimate = zxcvbn::zxcvbn(&password, &[]).unwrap();
    diesel::update(users.filter(email.eq(mail)))
        .set((activated.eq(true), pass_strength.eq(format!("{}", estimate.score())), crack_time.eq(format!("{}", estimate.crack_times().online_no_throttling_10_per_second()))))
        .execute(&connection)
        .unwrap();
}

pub fn get_users() -> Vec<User> {
    let connection = establish_connection();
    users.load::<User>(&connection)
        .unwrap()
}
