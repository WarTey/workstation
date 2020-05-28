use rocket::http::{Cookie, Cookies};
use rocket::outcome::IntoOutcome;
use rocket::request::{FromRequest, Request, Outcome, Form};
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use rocket_contrib::json::Json;

use crate::models::User;
use crate::database::{delete_user, update_user_link, update_user_approbation, update_super_user, check_approbation, check_email, check_super_user, get_super_users, get_link_from_email, get_not_all_users};
use crate::regex::{regex_password, regex_email};

#[derive(Serialize)]
pub struct TemplateAdmin {
    users: Option<Vec<User>>
}

#[derive(Serialize)]
pub struct TemplateAnswer {
    success: bool,
    message: Option<String>,
    status: Option<bool>,
    token: Option<String>
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
pub fn login(mut cookies: Cookies<'_>, form: Form<Login>) -> Json<TemplateAnswer> {
    if !regex_email(form.email.to_string()) || !regex_password(form.password.to_string()) || !check_email(form.email.to_string()) || !check_super_user(form.email.to_string()) {
        Json(TemplateAnswer {
            success: false,
            message: Some("Invalid form.".to_string()),
            status: None,
            token: None
        })
    } else {
        cookies.add_private(Cookie::new("admin_email", form.email.to_string()));
        Json(TemplateAnswer {
            success: true,
            message: None,
            status: None,
            token: None
        })
    }
}

#[post("/logout")]
pub fn logout(mut cookies: Cookies<'_>) -> Redirect {
    cookies.remove_private(Cookie::named("admin_email"));
    Redirect::to(uri!(admin))
}

#[post("/reset_user", data = "<form>")]
pub fn reset_user(admin: Admin, form: Form<ManageUser>) -> Json<TemplateAnswer> {
    if !regex_email(form.email.to_string()) || !check_email(form.email.to_string()) || form.email == admin.0 {
        Json(TemplateAnswer {
            success: false,
            message: Some("Invalid email.".to_string()),
            status: None,
            token: None
        })
    } else {
        update_user_link(form.email.to_string());
        Json(TemplateAnswer {
            success: true,
            message: Some("Link sent.".to_string()),
            status: None,
            token: Some(get_link_from_email(form.email.to_string()))
        })
    }
}

#[post("/delete_user", data = "<form>")]
pub fn remove_user(admin: Admin, form: Form<ManageUser>) -> Json<TemplateAnswer> {
    if !regex_email(form.email.to_string()) || !check_email(form.email.to_string()) || form.email == admin.0 {
        Json(TemplateAnswer {
            success: false,
            message: Some("Invalid email.".to_string()),
            status: None,
            token: None
        })
    } else {
        delete_user(form.email.to_string());
        Json(TemplateAnswer {
            success: true,
            message: Some("User removed.".to_string()),
            status: None,
            token: None
        })
    }
}

#[post("/approve_user", data = "<form>")]
pub fn approve_user(admin: Admin, form: Form<ManageUser>) -> Json<TemplateAnswer> {
    if !regex_email(form.email.to_string()) || !check_email(form.email.to_string()) || form.email == admin.0 {
        Json(TemplateAnswer {
            success: false,
            message: Some("Invalid email.".to_string()),
            status: Some(false),
            token: None
        })
    } else {
        update_user_approbation(form.email.to_string(), !check_approbation(get_link_from_email(form.email.to_string())));
        Json(TemplateAnswer {
            success: true,
            message: Some("User status updated.".to_string()),
            status: Some(check_approbation(get_link_from_email(form.email.to_string()))),
            token: None
        })
    }
}

#[post("/super_user", data = "<form>")]
pub fn super_user(admin: Admin, form: Form<ManageUser>) -> Json<TemplateAnswer> {
    if !regex_email(form.email.to_string()) || !check_email(form.email.to_string()) || form.email == admin.0 {
        Json(TemplateAnswer {
            success: false,
            message: Some("Invalid email.".to_string()),
            status: Some(false),
            token: None
        })
    } else if check_super_user(form.email.to_string()) && get_super_users() > 1 {
        update_super_user(form.email.to_string(), false);
        Json(TemplateAnswer {
            success: true,
            message: Some("User status updated.".to_string()),
            status: Some(false),
            token: None
        })
    } else if !check_super_user(form.email.to_string()) {
        update_super_user(form.email.to_string(), true);
        Json(TemplateAnswer {
            success: true,
            message: Some("User status updated.".to_string()),
            status: Some(true),
            token: None
        })
    } else {
        Json(TemplateAnswer {
            success: false,
            message: Some("There's only one administrator left.".to_string()),
            status: Some(false),
            token: None
        })
    }
}

#[get("/admin")]
pub fn admin(admin: Option<Admin>) -> Template {
    Template::render(if admin.is_some() {
        "admin"
    } else {
        "login"
    }, TemplateAdmin {
        users: match admin {
            Some(email) => Some(get_not_all_users(email.0)),
            _ => None,
        }
    })
}
