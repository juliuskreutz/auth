use yarte::Template;

#[derive(Template)]
#[template(path = "home.hbs")]
pub struct HomeTemplate {
    email: String,
    password: String,
    valid: bool,
}

impl HomeTemplate {
    pub fn invalid() -> Self {
        HomeTemplate {
            email: "".to_owned(),
            password: "".to_owned(),
            valid: false,
        }
    }

    pub fn valid(email: String, password: String) -> Self {
        HomeTemplate {
            email,
            password,
            valid: true,
        }
    }
}
