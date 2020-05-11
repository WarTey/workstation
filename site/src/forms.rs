use rocket::response::Redirect;
use rocket::request::Form;

#[derive(FromForm)]
pub struct TakeUser {
    firstname: String,
    lastname: String,
    password: String,
    repassword: String
}

#[derive(FromForm)]
pub struct CreatePassword {
    email: String,
    password: String,
    confirmpassword: String
}

#[post("/take_user", data = "<form>")]
pub fn take_user(form: Form<TakeUser>) -> Result<Redirect, String> {
    if form.firstname.len() > 50 {
        return Err(format!("Error length form.firstname!"));
    }

    if form.lastname.len() > 50 {
        return Err(format!("Error length form.lastname!"));
    }

    if form.password.len() < 13 {
        return Err(format!("Error length form.password!"));
    }

    if form.repassword != form.password {
        return Err(format!("Error repassword!"));
    }

    Ok(Redirect::to("/hello/success"))
}

#[post("/create_password", data = "<form>")]
pub fn create_password(form: Form<CreatePassword>) -> Result<Redirect, String> {
    if form.email.len() > 100 {
        return Err(format!("Error length form.firstname!"));
    }

    if form.password.len() < 13 {
        return Err(format!("Error length form.password!"));
    }

    if form.confirmpassword != form.password {
        return Err(format!("Error form.confirmpassword!"));
    }

    Ok(Redirect::to("/hello/success"))
}