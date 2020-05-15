use rocket::response::{Redirect, Flash};
use rocket::request::Form;
use regex::Regex;

use crate::database::{add_user, update_user_link, check_email, get_link_from_email, update_user_password, create_user_password};

#[derive(FromForm)]
pub struct ResetUser {
    email: String
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

#[derive(FromForm)]
pub struct CreateUser {
    email: String,
    firstname: String,
    lastname: String
}

fn regex_password(password: String) -> bool {
    let regex_length = Regex::new(r"^.{13,100}$").unwrap();
    let regex_letter = Regex::new(r"[a-z]").unwrap();
    let regex_capital = Regex::new(r"[A-Z]").unwrap();
    let regex_number = Regex::new(r"\d").unwrap();
    let regex_special = Regex::new(r"\W|_").unwrap();
    regex_length.is_match(&password) && regex_letter.is_match(&password) && regex_capital.is_match(&password) && regex_number.is_match(&password) && regex_special.is_match(&password)
}

fn regex_email(mail: String) -> bool {
    Regex::new(r"^\S{2,68}@\S{2,15}[.]\S{2,15}$").unwrap().is_match(&mail)
}

fn regex_name(name: String) -> bool {
    Regex::new(r"^\S{2,50}$").unwrap().is_match(&name)
}

#[post("/send_link", data = "<form>")]
pub fn send_link(form: Form<ResetUser>) -> Flash<Redirect> {
    if !regex_email(form.email.to_string()) || !check_email(form.email.to_string()) {
        Flash::error(Redirect::to(uri!(super::incorrect_link)), "Invalid email.")
    } else {
        update_user_link(form.email.to_string());
        Flash::success(Redirect::to(uri!(super::incorrect_link)), "Link sent.")
    }
}

#[post("/edit_user", data = "<form>")]
pub fn edit_user(form: Form<EditUser>) -> Flash<Redirect> {
    if !regex_email(form.email.to_string()) || !regex_password(form.old_password.to_string()) || !regex_password(form.password.to_string()) || form.repassword != form.password || !check_email(form.email.to_string()) {
        Flash::error(Redirect::to(uri!(super::get: get_link_from_email(form.email.to_string()))), "Invalid form.")
    } else {
        update_user_password(form.email.to_string(), form.password.to_string());
        Flash::success(Redirect::to(uri!(super::get: get_link_from_email(form.email.to_string()))), "Password updated.")
    }
}

#[post("/create_user", data = "<form>")]
pub fn create_user(form: Form<CreateUser>) -> Flash<Redirect> {
    if !regex_email(form.email.to_string()) || check_email(form.email.to_string()) || !regex_name(form.firstname.to_string()) || !regex_name(form.lastname.to_string()) {
        Flash::error(Redirect::to(uri!(super::create)), "Invalid form.")
    } else {
        add_user(form.firstname.to_string(), form.lastname.to_string(), form.email.to_string());
        Flash::success(Redirect::to(uri!(super::create)), "Request sent.")
    }
}

#[post("/create_password", data = "<form>")]
pub fn create_password(form: Form<CreatePassword>) -> Flash<Redirect> {
    if !regex_email(form.email.to_string()) || !regex_password(form.password.to_string()) || form.repassword != form.password || !check_email(form.email.to_string()) {
        Flash::error(Redirect::to(uri!(super::get: get_link_from_email(form.email.to_string()))), "Invalid form.")
    } else {
        create_user_password(form.email.to_string(), form.password.to_string());
        Flash::success(Redirect::to(uri!(super::get: get_link_from_email(form.email.to_string()))), "Password created.")
    }
}
