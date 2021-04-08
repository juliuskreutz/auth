use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    email: String,
    password: String,
}

impl User {
    pub fn new(email: String, password: String) -> Self {
        User { email, password }
    }

    pub fn email(&self) -> &String {
        &self.email
    }

    pub fn password(&self) -> &String {
        &self.password
    }
}

#[derive(Debug, Clone)]
pub struct Confirmation {
    uuid: String,
    email: String,
    password: String,
}

impl Confirmation {
    pub fn new(uuid: String, email: String, password: String) -> Self {
        Confirmation {
            uuid,
            email,
            password,
        }
    }

    pub fn uuid(&self) -> &String {
        &self.uuid
    }

    pub fn email(&self) -> &String {
        &self.email
    }

    pub fn password(&self) -> &String {
        &self.password
    }
}
