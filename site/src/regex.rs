use regex::Regex;

pub fn regex_password(password: String) -> bool {
    let regex_length = Regex::new(r"^.{13,100}$").unwrap();
    let regex_letter = Regex::new(r"[a-z]").unwrap();
    let regex_capital = Regex::new(r"[A-Z]").unwrap();
    let regex_number = Regex::new(r"\d").unwrap();
    let regex_special = Regex::new(r"\W|_").unwrap();
    regex_length.is_match(&password) && regex_letter.is_match(&password) && regex_capital.is_match(&password) && regex_number.is_match(&password) && regex_special.is_match(&password)
}

pub fn regex_email(mail: String) -> bool {
    Regex::new(r"^\S{2,68}@\S{2,15}[.]\S{2,15}$").unwrap().is_match(&mail)
}

pub fn regex_name(name: String) -> bool {
    Regex::new(r"^.{2,50}$").unwrap().is_match(&name)
}