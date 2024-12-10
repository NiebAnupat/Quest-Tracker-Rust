use anyhow::Result;
use std::sync::Arc;
use crate::domain::repositories::quest_viewing::QuestViewingRepository;
use crate::domain::value_objects::board_checking_filter::BoardCheckingFilter;
use crate::domain::value_objects::quest_model::QuestModel;

pub struct QuestViewingUseCase<T>
where
    T: QuestViewingRepository + Send + Sync,
{
    pub quest_viewing_repository: Arc<T>,
}

impl<T> QuestViewingUseCase<T>
where
    T: QuestViewingRepository + Send + Sync,
{
    pub fn new(quest_viewing_repository: Arc<T>) -> Self {
        Self { quest_viewing_repository }
    }

    async fn view_details(&self, quest_id: i32) -> Result<QuestModel> {
        unimplemented!()
    }
    async fn board_checking(&self, filter: &BoardCheckingFilter) -> Result<Vec<QuestModel>> {
        unimplemented!()
    }
    async fn adventurers_counting_by_quest_id(&self, quest_id: i32) -> Result<i64> {
        unimplemented!()
    }
}