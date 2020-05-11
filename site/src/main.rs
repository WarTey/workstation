#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

mod models;
mod schema;
mod forms;
mod database;

use std::collections::HashMap;

use rocket::response::Redirect;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

use database::{check_link, get_email_from_link};

#[get("/<link>")]
fn get(link: String) -> Template {
    let mut context: HashMap<&str, String> = HashMap::new();
    if link.len() == 32 && check_link(format!("{}", link)) {
        context.insert("email", get_email_from_link(link).unwrap());
        Template::render("edit", context)
    } else {
        Template::render("url-lost", context)
    }
}

#[catch(404)]
fn not_found() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("url-lost", context)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![get, forms::take_user, forms::send_link])
        .mount("/static", StaticFiles::from("static"))
        .attach(Template::fairing())
        .register(catchers![not_found])
}

fn main() {
    rocket().launch();
}
