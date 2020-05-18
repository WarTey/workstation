#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;

mod models;
mod schema;
mod forms;
mod database;

use std::collections::HashMap;

use rocket::outcome::IntoOutcome;
use rocket::response::Redirect;
use rocket::request::{FlashMessage, FromRequest, Request, Outcome};
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use rocket_contrib::json::Json;

use models::User;
use database::{check_link, get_email_from_link, get_users, check_status, check_approbation};

#[derive(Serialize)]
struct TemplateContext {
    email: String,
    flash: Option<(String, String)>,
    activated: bool
}

#[derive(Serialize)]
struct TemplateAdmin {
    users: Option<Vec<User>>,
    flash: Option<(String, String)>
}

#[derive(Serialize, Deserialize)]
struct UsersStatistics {
    email: String,
    pass_strength: String,
    crack_time: String
}

struct Admin(usize);

impl<'a, 'r> FromRequest<'a, 'r> for Admin {
    type Error = std::convert::Infallible;

    fn from_request(request: &'a Request<'r>) -> Outcome<Admin, Self::Error> {
        println!("FromRequest");
        request.cookies()
            .get_private("admin_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|id| Admin(id))
            .or_forward(())
    }
}

fn flash_message(flash: Option<FlashMessage>) -> Option<(String, String)> {
    flash.map(|flash| {
        let name = flash.name().to_string();
        let message = flash.msg().to_string();
        (name, message)
    })
}

#[get("/statistics")]
fn statistics() -> Option<Json<Vec<UsersStatistics>>> {
    let mut statistics: Vec<UsersStatistics> = Vec::new();
    // Restreindre l'accès au ScreenSaver ?
    for user in get_users() {
        statistics.push(UsersStatistics {
            email: user.email,
            pass_strength: user.pass_strength,
            crack_time: user.crack_time
        });
    }
    Some(Json(statistics))
}

#[get("/admin")]
fn admin(admin: Option<Admin>, flash: Option<FlashMessage>) -> Template {
    //let mut context: HashMap<&str, Option<(String, String)>> = HashMap::new();
    //context.insert("flash", flash_message(flash));
    let context = TemplateAdmin {
        users: Some(get_users()),
        flash: flash_message(flash)
    };
    if admin.is_some() {
        println!("admin_user");
        Template::render("admin", context)
    } else {
        println!("admin_page");
        Template::render("admin", context)
    }
}

#[get("/create")]
fn create(flash: Option<FlashMessage>) -> Template {
    let mut context: HashMap<&str, Option<(String, String)>> = HashMap::new();
    context.insert("flash", flash_message(flash));
    Template::render("create", context)
}

#[get("/<link>")]
fn get(link: String, flash: Option<FlashMessage>) -> Result<Template, Redirect> {
    if link.len() == 32 && check_link(link.to_string()) && check_approbation(link.to_string()) {
        let token = link.clone();
        Ok(Template::render("edit", TemplateContext {
            email: get_email_from_link(link),
            flash: flash_message(flash),
            activated: check_status(token)
        }))
    } else {
        Err(Redirect::to(uri!(incorrect_link)))
    }
}

#[get("/incorrect_link")]
fn incorrect_link(flash: Option<FlashMessage>) -> Template {
    let mut context: HashMap<&str, Option<(String, String)>> = HashMap::new();
    context.insert("flash", flash_message(flash));
    Template::render("url-lost", context)
}

#[catch(404)]
fn not_found() -> Redirect {
    Redirect::to(uri!(incorrect_link))
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![statistics, admin, create, get, incorrect_link, forms::create_password, forms::create_user, forms::edit_user, forms::send_link, forms::login, forms::logout])
        .mount("/static", StaticFiles::from("static"))
        .attach(Template::fairing())
        .register(catchers![not_found])
}

fn main() {
    rocket().launch();
}
