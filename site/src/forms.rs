use rocket::response::Redirect;
use rocket::request::Form;

use crate::database::update_user_activation;

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
pub fn send_link(form: Form<ResetUser>) -> Result<Redirect, String> {
    if form.email.len() > 100 {
        return Err(format!("Error length form.email!"));
    }

    update_user_activation(format!("{}", form.email), false);
    Ok(Redirect::to("/"))
}