use crate::schema::users;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub token: String,
    pub activated: bool,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub firstname: &'a str,
    pub lastname: &'a str,
}