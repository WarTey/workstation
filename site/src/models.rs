use crate::schema::users;

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub pass_strength: String,
    pub crack_time: String,
    pub token: String,
    pub activated: bool,
    pub approved: bool,
    pub super_user: bool,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub firstname: &'a str,
    pub lastname: &'a str,
    pub email: &'a str,
    pub pass_strength: String,
    pub crack_time: String,
    pub activated: bool,
    pub approved: bool,
    pub super_user: bool,
}