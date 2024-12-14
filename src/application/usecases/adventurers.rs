use std::sync::Arc;
use anyhow::Result;
use tracing::info;
use crate::domain::repositories::adventurers::AdventurersRepository;
use crate::domain::value_objects::adventurer_model::RegisterAdventurerModel;
use crate::infrastructure::argon2_hashing::hash;

pub struct AdventurersUseCase<T>
where
    T: AdventurersRepository + Send + Sync,
{
    adventurers_repository: Arc<T>,
}

const SERVICE_CONTEXT: &str = "ADVENTURERS_USE_CASE";
impl<T> AdventurersUseCase<T>
where
    T: AdventurersRepository + Send + Sync,
{
    pub fn new(adventurers_repository: Arc<T>) -> Self {
        Self { adventurers_repository }
    }

    pub async fn register(&self, mut register_adventurer_model: RegisterAdventurerModel) -> Result<i32> {
        const ACTION_NAME: &str = "REGISTER";
        info!("START - {}: {} - server_time: {}", SERVICE_CONTEXT, ACTION_NAME, chrono::Utc::now());

        // check for existing username
        let username_exists = self.adventurers_repository.find_by_username(register_adventurer_model.username.clone()).await.is_ok();
        if username_exists {
            return Err(anyhow::anyhow!("มีผู้นักผจญภัยท่านอื่นใช้ชื่อผู้ใช้งานนี้แล้ว"));
        }

        let hashed = hash(&register_adventurer_model.password)?;
        register_adventurer_model.password = hashed;

        let register_entity = register_adventurer_model.to_entity();
        let id = self.adventurers_repository.register(register_entity).await?;

        info!("SUCCESS - {}: {} - id={} - server_time: {}", SERVICE_CONTEXT, ACTION_NAME, id, chrono::Utc::now());
        Ok(id)
    }
}
