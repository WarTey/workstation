use rocket_contrib::json::Json;

#[derive(Serialize, Deserialize)]
pub struct Statistic {
    email: String,
    pass_strength: String,
    crack_time: String
}

#[get("/statistics")]
pub fn statistics() -> Option<Json<Vec<Statistic>>> {
    let mut statistics: Vec<Statistic> = Vec::new();
    for user in crate::database::get_users() {
        statistics.push(Statistic {
            email: user.email,
            pass_strength: user.pass_strength,
            crack_time: user.crack_time
        });
    }
    Some(Json(statistics))
}
