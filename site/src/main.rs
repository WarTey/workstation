#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;

mod models;
mod schema;
mod forms;
mod database;

use std::collections::HashMap;

use rocket::response::Redirect;
use rocket::request::FlashMessage;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use rocket_contrib::json::Json;

use database::{check_link, get_email_from_link, get_users, check_status};

#[derive(Serialize)]
struct TemplateContext {
    email: String,
    flash: Option<(String, String)>,
    activated: bool
}

#[derive(Debug, Serialize, Deserialize)]
struct UsersStatistics {
    email: String,
    pass_strength: String,
    crack_time: String
}

#[get("/statistics")]
fn statistics() -> Option<Json<Vec<UsersStatistics>>> {
    let mut statistics: Vec<UsersStatistics> = Vec::new();
    for user in get_users() {
        statistics.push(UsersStatistics {
            email: user.email,
            pass_strength: user.pass_strength,
            crack_time: user.crack_time
        });
    }
    Some(Json(statistics))
}

#[get("/create")]
fn create(flash: Option<FlashMessage>) -> Template {
    let mut context: HashMap<&str, (String, String)> = HashMap::new();
    if flash.is_some() {
        context.insert("flash", flash.map(|flash| {
            let name = flash.name().to_string();
            let message = flash.msg().to_string();
            (name, message)
        }).unwrap());
    }
    Template::render("create", context)
}

#[get("/<link>")]
fn get(link: String, flash: Option<FlashMessage>) -> Result<Template, Redirect> {
    if link.len() == 32 && check_link(link.to_string()) {
        let token = link.clone();
        let context = if flash.is_some() {
            TemplateContext {
                email: get_email_from_link(link),
                flash: flash.map(|flash| {
                    let name = flash.name().to_string();
                    let message = flash.msg().to_string();
                    (name, message)
                }),
                activated: check_status(token)
            }
        } else {
            TemplateContext {
                email: get_email_from_link(link),
                flash: None,
                activated: check_status(token)
            }
        };
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
        .mount("/", routes![statistics, create, get, incorrect_link, forms::create_password, forms::create_user, forms::edit_user, forms::send_link])
        .mount("/static", StaticFiles::from("static"))
        .attach(Template::fairing())
        .register(catchers![not_found])
}

fn main() {
    rocket().launch();
}
