use crate::types::Entity;
use std::{fmt};

impl TryFrom<Vec<String>> for Entity {
    type Error = &'static str;

    fn try_from(words: Vec<String>) -> Result<Self, Self::Error> {
        let id = match words.get(0) {
            Some(s) => match s.parse::<u32>() {
                Ok(v) => v,
                Err(_) => return Err("id must be a number"),
            },
            None => return Err("id is required"),
        };
        let username = match words.get(1) {
            Some(s) => s,
            None => return Err("username is required"),
        };
        let email = match words.get(2) {
            Some(s) => s,
            None => return Err("email is required"),
        };
        Ok(Entity {
            id,
            username: username.to_string(),
            email: email.to_string(),
        })
    }
}

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.id, self.username, self.email)
    }
}
