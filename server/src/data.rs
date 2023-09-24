use std::{collections::HashMap, hash::Hash};

use crate::Credentials;

#[derive(Debug)]
pub struct Database {
    users: HashMap<String, User>,
    sessions: HashMap<Token, uuid::Uuid>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Token(uuid::Uuid);

impl ToString for Token {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

/* impl Hash for Token {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
} */

#[derive(Debug)]
pub struct User {
    id: uuid::Uuid,
    name: String,
    password_hash: String,
}

impl Database {
    /// Generate token for existing user or newly created user
    pub fn login_user(&mut self, user: Credentials) -> Result<Token, anyhow::Error> {
        let user = self.users.entry(user.name).or_insert(User {
            name: user.name,
            password_hash: user.password,
            id: uuid::Uuid::new_v4(),
        });
        let token = Token(uuid::Uuid::new_v4());
        self.sessions.insert(token, user.id);
        Ok(token)
    }
    /// Delete user session
    pub fn logout_user(&mut self, t: &Token) -> Result<(), anyhow::Error> {
        self.sessions.remove(t);
        Ok(())
    }
}
