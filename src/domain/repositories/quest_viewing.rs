use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;
use crate::domain::entities::quests::QuestEntity;
use crate::domain::value_objects::board_checking_filter::BoardCheckingFilter;

#[async_trait]
#[automock]
pub trait QuestViewingRepository {
    async fn view_details(&self, quest_id: i32) -> Result<QuestEntity>;
    async fn board_checking(&self, filter: &BoardCheckingFilter) -> Result<Vec<QuestEntity>>;
    async fn adventurers_counting_by_quest_id(&self, quest_id: i32) -> Result<i64>;
}