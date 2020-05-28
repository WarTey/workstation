#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate openssl;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate serde_derive;

mod models;
mod schema;
mod regex;
mod statistics;
mod admin;
mod user;
mod wifi;
mod database;

use rocket::fairing::AdHoc;
use rocket::response::Redirect;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

#[catch(404)]
fn not_found() -> Redirect {
    Redirect::to(uri!(user::incorrect_link))
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(AdHoc::on_attach("Database Migrations", database::run_db_migrations))
        .mount("/", routes![statistics::statistics, admin::login, admin::logout, admin::reset_user, admin::remove_user, admin::approve_user, admin::super_user, admin::admin, user::create_user, user::create_password, user::edit_user, user::send_link, user::create, user::get, user::incorrect_link])
        .mount("/static", StaticFiles::from("static"))
        .attach(Template::fairing())
        .register(catchers![not_found])
}

fn main() {
    rocket().launch();
}
