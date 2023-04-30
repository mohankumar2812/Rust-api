pub fn get_valid_mail(mail: &str) -> TypeValidMail {
    match Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([a-z0-9]+)*\.[a-z]{2,6})")
    {
        Result::Ok(regex) => match regex.is_match(mail) {
            true => TypeValidMail::Ok,
            false => TypeValidMail::BadMail,
        },
        Err(_) => TypeValidMail::BadMail,
    }
}