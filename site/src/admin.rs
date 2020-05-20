use rocket::http::{Cookie, Cookies};
use rocket::outcome::IntoOutcome;
use rocket::request::{FlashMessage, FromRequest, Request, Outcome, Form};
use rocket::response::{Redirect, Flash};
use rocket_contrib::templates::Template;

use crate::models::User;
use crate::database::{delete_user, update_user_link, update_user_approbation, update_super_user, check_approbation, check_email, check_super_user, get_super_users, get_link_from_email, get_not_all_users};
use crate::regex::{regex_password, regex_email};

#[derive(Serialize)]
pub struct TemplateAdmin {
    users: Option<Vec<User>>,
    flash: Option<(String, String)>
}

#[derive(FromForm)]
pub struct ManageUser {
    email: String
}

#[derive(FromForm)]
pub struct Login {
    email: String,
    password: String
}

#[derive(Debug)]
pub struct Admin(String);

impl<'a, 'r> FromRequest<'a, 'r> for Admin {
    type Error = std::convert::Infallible;

    fn from_request(request: &'a Request<'r>) -> Outcome<Admin, Self::Error> {
        request.cookies()
            .get_private("admin_email")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|id| Admin(id))
            .or_forward(())
    }
}

#[post("/login", data = "<form>")]
pub fn login(mut cookies: Cookies<'_>, form: Form<Login>) -> Flash<Redirect> {
    if !regex_email(form.email.to_string()) || !regex_password(form.password.to_string()) || !check_email(form.email.to_string()) || !check_super_user(form.email.to_string()) {
        Flash::error(Redirect::to(uri!(admin)), "Invalid form.")
    } else {
        cookies.add_private(Cookie::new("admin_email", form.email.to_string()));
        Flash::success(Redirect::to(uri!(admin)), "Welcome administrator.")
    }
}

#[post("/logout")]
pub fn logout(mut cookies: Cookies<'_>) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named("admin_email"));
    Flash::success(Redirect::to(uri!(admin)), "Successfully logged out.")
}

#[post("/reset_user", data = "<form>")]
pub fn reset_user(admin: Admin, form: Form<ManageUser>) -> Flash<Redirect> {
    if !regex_email(form.email.to_string()) || !check_email(form.email.to_string()) || form.email == admin.0 {
        Flash::error(Redirect::to(uri!(admin)), "Invalid email.")
    } else {
        update_user_link(form.email.to_string());
        Flash::success(Redirect::to(uri!(admin)), "Link sent.")
    }
}

#[post("/delete_user", data = "<form>")]
pub fn remove_user(admin: Admin, form: Form<ManageUser>) -> Flash<Redirect> {
    if !regex_email(form.email.to_string()) || !check_email(form.email.to_string()) || form.email == admin.0 {
        Flash::error(Redirect::to(uri!(admin)), "Invalid email.")
    } else {
        delete_user(form.email.to_string());
        Flash::success(Redirect::to(uri!(admin)), "User removed.")
    }
}

#[post("/approve_user", data = "<form>")]
pub fn approve_user(admin: Admin, form: Form<ManageUser>) -> Flash<Redirect> {
    if !regex_email(form.email.to_string()) || !check_email(form.email.to_string()) || form.email == admin.0 {
        Flash::error(Redirect::to(uri!(admin)), "Invalid email.")
    } else {
        update_user_approbation(form.email.to_string(), !check_approbation(get_link_from_email(form.email.to_string())));
        Flash::success(Redirect::to(uri!(admin)), "User status updated.")
    }
}

#[post("/super_user", data = "<form>")]
pub fn super_user(admin: Admin, form: Form<ManageUser>) -> Flash<Redirect> {
    if !regex_email(form.email.to_string()) || !check_email(form.email.to_string()) || form.email == admin.0 {
        Flash::error(Redirect::to(uri!(admin)), "Invalid email.")
    } else if check_super_user(form.email.to_string()) && get_super_users() > 1 {
        update_super_user(form.email.to_string(), false);
        Flash::success(Redirect::to(uri!(admin)), "User status updated.")
    } else if !check_super_user(form.email.to_string()) {
        update_super_user(form.email.to_string(), true);
        Flash::success(Redirect::to(uri!(admin)), "User status updated.")
    } else {
        Flash::error(Redirect::to(uri!(admin)), "There's only one administrator left.")
    }
}

#[get("/admin")]
pub fn admin(admin: Option<Admin>, flash: Option<FlashMessage>) -> Template {
    Template::render(if admin.is_some() {
        "admin"
    } else {
        "login"
    }, TemplateAdmin {
        users: match admin {
            Some(email) => Some(get_not_all_users(email.0)),
            _ => None,
        },
        flash: super::flash_message(flash)
    })
}
