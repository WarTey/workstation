use rocket::response::{Redirect, Flash};
use rocket::request::Form;

use crate::database::{update_user_activation, check_email};

#[derive(FromForm)]
pub struct TakeUser {
    firstname: String,
    lastname: String,
    email: String,
    password: String,
    repassword: String
}

#[derive(FromForm, Debug)]
pub struct ResetUser {
    email: String
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
