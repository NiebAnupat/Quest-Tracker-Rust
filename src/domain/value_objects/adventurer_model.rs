use serde::{Deserialize, Serialize};
use crate::domain::entities::adventurers::RegisterAdventurerEntity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterAdventurerModel {
    pub username: String,
    pub password: String,
}

impl RegisterAdventurerModel {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }

    pub fn to_entity(&self) -> RegisterAdventurerEntity {
        let now = chrono::Utc::now().naive_utc();
        RegisterAdventurerEntity {
            username: self.username.clone(),
            password: self.password.clone(),
            created_at: now,
            updated_at: now,
        }
    }
}