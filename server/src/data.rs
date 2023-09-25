use std::{collections::HashMap, hash::Hash};

use crate::server::Credentials;

#[derive(Debug)]
pub struct Database {
    pub users: HashMap<String, User>,
    pub sessions: HashMap<Token, uuid::Uuid>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Token(uuid::Uuid);

impl ToString for Token {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl TryFrom<String> for Token {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let uid: uuid::Uuid = uuid::Uuid::try_parse(&value).map_err(|e| {
            format!(
                "Failed to parse uuid from string: {}, couse of: {}",
                value, e
            )
        })?;
        Ok(Self(uid))
    }
}

#[derive(Debug)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub password_hash: String,
}

impl Database {
    /// Generate token for existing user or newly created user
    pub fn login_user(&mut self, user: Credentials) -> Result<Token, anyhow::Error> {
        let user = self.users.entry(user.name.clone()).or_insert(User {
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
    /// List all users
    pub fn list_all_users(&self) -> Result<Vec<&User>, anyhow::Error> {
        Ok(self.users.values().collect())
    }
}
