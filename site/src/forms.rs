use rocket::response::{Redirect, Flash};
use rocket::request::Form;
use regex::Regex;

use crate::database::{update_user_activation, check_email, get_link_from_email};

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
        update_user_activation(format!("{}", form.email), false);
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
        Flash::success(Redirect::to(uri!(super::get: get_link_from_email(format!("{}", form.email)))), "Password updated.")
    }
}
