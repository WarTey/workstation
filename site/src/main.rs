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
use rocket::request::FlashMessage;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

use database::{check_link, get_email_from_link};

#[get("/<link>")]
fn get(link: String) -> Result<Template, Redirect> {
    let mut context: HashMap<&str, String> = HashMap::new();
    if link.len() == 32 && check_link(format!("{}", link)) {
        context.insert("email", get_email_from_link(link).unwrap());
        Ok(Template::render("edit", context))
    } else {
        Err(Redirect::to(uri!(incorrect_link)))
    }
}

#[get("/incorrect_link")]
fn incorrect_link(flash: Option<FlashMessage>) -> Template {
    let mut context: HashMap<&str, (String, String)> = HashMap::new();
    if flash.is_some() {
        context.insert("flash", flash.map(|flash| {
            let name = flash.name().to_string();
            let message = flash.msg().to_string();
            (name, message)
        }).unwrap());
    }
    Template::render("url-lost", context)
}

#[catch(404)]
fn not_found() -> Redirect {
    Redirect::to(uri!(incorrect_link))
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![get, incorrect_link, forms::take_user, forms::send_link])
        .mount("/static", StaticFiles::from("static"))
        .attach(Template::fairing())
        .register(catchers![not_found])
}

fn main() {
    rocket().launch();
}
