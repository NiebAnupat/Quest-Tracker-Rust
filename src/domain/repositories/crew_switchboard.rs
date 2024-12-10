use crate::domain::value_objects::quest_adventurer_junction::QuestAdventurerJunction;
use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;

#[async_trait]
#[automock]
pub trait CrewSwitchboardRepository {
    async fn join(&self, junction_body:QuestAdventurerJunction) -> Result<()>;
    async fn leave(&self, junction_body:QuestAdventurerJunction) -> Result<()>;
}