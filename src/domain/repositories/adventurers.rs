use crate::domain::value_objects::adventurer_model::RegisterAdventurerModel;
use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;
use crate::domain::entities::adventurers::AdventurerEntity;

#[async_trait]
#[automock]
pub trait AdventurersRepository {
    async fn register(&self, register_adventurer_model: RegisterAdventurerModel) -> Result<i32>;
    async fn find_by_username(&self, username: String) -> Result<AdventurerEntity>;
}