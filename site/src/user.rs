use rocket::request::{FlashMessage, Form};
use rocket::response::{Redirect, Flash};
use rocket_contrib::templates::Template;
use std::collections::HashMap;

use crate::database::{add_user, update_user_link, update_user_password, create_user_password, check_email, check_link, check_status, check_approbation, get_link_from_email, get_email_from_link};
use crate::regex::{regex_name, regex_email, regex_password};

#[derive(Serialize)]
struct TemplateContext {
    email: String,
    flash: Option<(String, String)>,
    activated: bool
}

#[derive(FromForm)]
pub struct ResetUser {
    email: String
}

#[derive(FromForm)]
pub struct CreateUser {
    email: String,
    firstname: String,
    lastname: String
}

#[derive(FromForm)]
pub struct EditUser {
    email: String,
    old_password: String,
    password: String,
    repassword: String
}

#[derive(FromForm)]
pub struct CreatePassword {
    email: String,
    password: String,
    repassword: String
}

#[post("/create_user", data = "<form>")]
pub fn create_user(form: Form<CreateUser>) -> Flash<Redirect> {
    if !regex_email(form.email.to_string()) || check_email(form.email.to_string()) || !regex_name(form.firstname.to_string()) || !regex_name(form.lastname.to_string()) {
        Flash::error(Redirect::to(uri!(create)), "Invalid form.")
    } else {
        add_user(form.firstname.to_string(), form.lastname.to_string(), form.email.to_string());
        Flash::success(Redirect::to(uri!(create)), "Request sent.")
    }
}

#[post("/create_password", data = "<form>")]
pub fn create_password(form: Form<CreatePassword>) -> Flash<Redirect> {
    if !regex_email(form.email.to_string()) || !regex_password(form.password.to_string()) || form.repassword != form.password || !check_email(form.email.to_string()) || !check_approbation(get_link_from_email(form.email.to_string())) {
        Flash::error(Redirect::to(uri!(get: get_link_from_email(form.email.to_string()))), "Invalid form.")
    } else {
        create_user_password(form.email.to_string(), form.password.to_string());
        Flash::success(Redirect::to(uri!(get: get_link_from_email(form.email.to_string()))), "Password created.")
    }
}

#[post("/edit_user", data = "<form>")]
pub fn edit_user(form: Form<EditUser>) -> Flash<Redirect> {
    if !regex_email(form.email.to_string()) || !regex_password(form.old_password.to_string()) || !regex_password(form.password.to_string()) || form.repassword != form.password || !check_email(form.email.to_string()) || !check_approbation(get_link_from_email(form.email.to_string())) {
        Flash::error(Redirect::to(uri!(get: get_link_from_email(form.email.to_string()))), "Invalid form.")
    } else {
        update_user_password(form.email.to_string(), form.password.to_string());
        Flash::success(Redirect::to(uri!(get: get_link_from_email(form.email.to_string()))), "Password updated.")
    }
}

#[post("/send_link", data = "<form>")]
pub fn send_link(form: Form<ResetUser>) -> Flash<Redirect> {
    if !regex_email(form.email.to_string()) || !check_email(form.email.to_string()) {
        Flash::error(Redirect::to(uri!(incorrect_link)), "Invalid email.")
    } else {
        update_user_link(form.email.to_string());
        Flash::success(Redirect::to(uri!(incorrect_link)), "Link sent.")
    }
}

#[get("/create")]
pub fn create(flash: Option<FlashMessage>) -> Template {
    let mut context: HashMap<&str, Option<(String, String)>> = HashMap::new();
    context.insert("flash", super::flash_message(flash));
    Template::render("create", context)
}

#[get("/<link>")]
pub fn get(link: String, flash: Option<FlashMessage>) -> Result<Template, Redirect> {
    if link.len() == 32 && check_link(link.to_string()) && check_approbation(link.to_string()) {
        let token = link.clone();
        Ok(Template::render("edit", TemplateContext {
            email: get_email_from_link(link),
            flash: super::flash_message(flash),
            activated: check_status(token)
        }))
    } else {
        Err(Redirect::to(uri!(incorrect_link)))
    }
}

#[get("/incorrect_link")]
pub fn incorrect_link(flash: Option<FlashMessage>) -> Template {
    let mut context: HashMap<&str, Option<(String, String)>> = HashMap::new();
    context.insert("flash", super::flash_message(flash));
    Template::render("url-lost", context)
}
