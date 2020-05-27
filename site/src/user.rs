use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use rocket_contrib::json::Json;
use std::collections::HashMap;

use crate::database::{add_user, update_user_link, update_user_password, create_user_password, check_email, check_link, check_status, check_approbation, get_link_from_email, get_email_from_link};
use crate::regex::{regex_name, regex_email, regex_password};

#[derive(Serialize)]
struct TemplateContext {
    email: String,
    activated: bool
}

#[derive(Serialize)]
pub struct TemplateAnswer {
    success: bool,
    message: Option<String>
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
    repassword: String,
    wifi: bool
}

#[derive(FromForm)]
pub struct CreatePassword {
    email: String,
    password: String,
    repassword: String
}

#[post("/create_user", data = "<form>")]
pub fn create_user(form: Form<CreateUser>) -> Json<TemplateAnswer> {
    if !regex_email(form.email.to_string()) || check_email(form.email.to_string()) || !regex_name(form.firstname.to_string()) || !regex_name(form.lastname.to_string()) {
        Json(TemplateAnswer {
            success: false,
            message: Some("Invalid form.".to_string())
        })
    } else {
        add_user(form.firstname.to_string(), form.lastname.to_string(), form.email.to_string());
        Json(TemplateAnswer {
            success: true,
            message: Some("Request sent.".to_string())
        })
    }
}

#[post("/create_password", data = "<form>")]
pub fn create_password(form: Form<CreatePassword>) -> Json<TemplateAnswer> {
    if !regex_email(form.email.to_string()) || !regex_password(form.password.to_string()) || form.repassword != form.password || !check_email(form.email.to_string()) || !check_approbation(get_link_from_email(form.email.to_string())) {
        Json(TemplateAnswer {
            success: false,
            message: Some("Invalid form.".to_string())
        })
    } else {
        create_user_password(form.email.to_string(), form.password.to_string());
        Json(TemplateAnswer {
            success: true,
            message: Some("Password created.".to_string())
        })
    }
}

#[post("/edit_user", data = "<form>")]
pub fn edit_user(form: Form<EditUser>) -> Json<TemplateAnswer> {
    if !regex_email(form.email.to_string()) || !regex_password(form.old_password.to_string()) || !regex_password(form.password.to_string()) || form.repassword != form.password || !check_email(form.email.to_string()) || !check_approbation(get_link_from_email(form.email.to_string())) {
        Json(TemplateAnswer {
            success: false,
            message: Some("Invalid form.".to_string())
        })
    } else {
        if form.wifi {
            Json(TemplateAnswer {
                success: true,
                message: Some("Wi-Fi password updated.".to_string())
            })
        } else {
            update_user_password(form.email.to_string(), form.password.to_string());
            Json(TemplateAnswer {
                success: true,
                message: Some("Password updated.".to_string())
            })
        }
    }
}

#[post("/send_link", data = "<form>")]
pub fn send_link(form: Form<ResetUser>) -> Json<TemplateAnswer> {
    if !regex_email(form.email.to_string()) || !check_email(form.email.to_string()) {
        Json(TemplateAnswer {
            success: false,
            message: Some("Invalid email.".to_string())
        })
    } else {
        update_user_link(form.email.to_string());
        Json(TemplateAnswer {
            success: true,
            message: Some("Link sent.".to_string())
        })
    }
}

#[get("/create")]
pub fn create() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("create", context)
}

#[get("/<link>")]
pub fn get(link: String) -> Result<Template, Redirect> {
    if link.len() == 32 && check_link(link.to_string()) && check_approbation(link.to_string()) {
        let token = link.clone();
        Ok(Template::render("edit", TemplateContext {
            email: get_email_from_link(link),
            activated: check_status(token)
        }))
    } else {
        Err(Redirect::to(uri!(incorrect_link)))
    }
}

#[get("/incorrect_link")]
pub fn incorrect_link() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("url-lost", context)
}
