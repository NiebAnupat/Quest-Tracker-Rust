use serde::{Deserialize, Serialize};
use crate::domain::entities::guild_commanders::RegisterGuildCommanderEntity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterGuildCommanderModel {
    pub username: String,
    pub password: String,
}

impl RegisterGuildCommanderModel {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }

    pub fn to_entity(&self) -> RegisterGuildCommanderEntity {
        let now = chrono::Utc::now().naive_utc();
        RegisterGuildCommanderEntity {
            username: self.username.clone(),
            password: self.password.clone(),
            created_at: now,
            updated_at: now,
        }
    }
}