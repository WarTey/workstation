use rocket::response::{Redirect, Flash};
use rocket::request::Form;
use regex::Regex;

use crate::database::{add_user, update_user_link, check_email, get_link_from_email, update_user_password};

#[derive(FromForm)]
pub struct TakeUser {
    firstname: String,
    lastname: String,
    email: String,
    password: String,
    repassword: String
}

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
pub struct CreateUser {
    email: String,
    firstname: String,
    lastname: String
}

#[post("/take_user", data = "<form>")]
pub fn take_user(form: Form<TakeUser>) -> Result<Redirect, String> {
    if form.firstname.len() > 50 {
        return Err(format!("Error length form.firstname!"));
    }

    if form.lastname.len() > 50 {
        return Err(format!("Error length form.lastname!"));
    }

    if form.email.len() > 100 {
        return Err(format!("Error length form.email!"));
    }

    if form.password.len() < 13 {
        return Err(format!("Error length form.password!"));
    }

    if form.repassword != form.password {
        return Err(format!("Error repassword!"));
    }

    Ok(Redirect::to("/hello/success"))
}

#[post("/send_link", data = "<form>")]
pub fn send_link(form: Form<ResetUser>) -> Flash<Redirect> {
    if form.email.len() > 100 || !check_email(format!("{}", form.email)) {
        Flash::error(Redirect::to(uri!(super::incorrect_link)), "Invalid email.")
    } else {
        update_user_link(format!("{}", form.email), false);
        Flash::success(Redirect::to(uri!(super::incorrect_link)), "Link sent.")
    }
}

fn regex_password(password: String) -> bool {
    let regex_letter = Regex::new(r"[a-z]").unwrap();
    let regex_capital = Regex::new(r"[A-Z]").unwrap();
    let regex_number = Regex::new(r"\d").unwrap();
    let regex_special = Regex::new(r"\W|_").unwrap();
    regex_letter.is_match(&password) && regex_capital.is_match(&password) && regex_number.is_match(&password) && regex_special.is_match(&password)
}

#[post("/edit_user", data = "<form>")]
pub fn edit_user(form: Form<EditUser>) -> Flash<Redirect> {
    if form.email.len() > 100 || form.old_password.len() < 13 || form.password.len() < 13 || !regex_password(format!("{}", form.password)) || form.repassword != form.password || !check_email(format!("{}", form.email)) {
        Flash::error(Redirect::to(uri!(super::get: get_link_from_email(format!("{}", form.email)))), "Invalid form.")
    } else {
        update_user_password(format!("{}", form.email), format!("{}", form.password));
        Flash::success(Redirect::to(uri!(super::get: get_link_from_email(format!("{}", form.email)))), "Password updated.")
    }
}

#[post("/create_user", data = "<form>")]
pub fn create_user(form: Form<CreateUser>) -> Flash<Redirect> {
    let regex_email = Regex::new(r"^\S+@\S+[.]+\S+$").unwrap();
    let regex_name = Regex::new(r"^\S*$").unwrap();
    if form.email.len() > 100 || form.firstname.len() > 50 || form.lastname.len() > 50 || !regex_email.is_match(&form.email) || !regex_name.is_match(&form.firstname) || !regex_name.is_match(&form.lastname) {
        Flash::error(Redirect::to(uri!(super::create)), "Invalid form.")
    } else {
        add_user(format!("{}", form.firstname), format!("{}", form.lastname), format!("{}", form.email));
        Flash::success(Redirect::to(uri!(super::create)), "Request sent.")
    }
}
