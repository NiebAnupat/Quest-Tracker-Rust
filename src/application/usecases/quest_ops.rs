use anyhow::Result;
use std::sync::Arc;
use crate::domain::repositories::quest_ops::QuestOpsRepository;
use crate::domain::repositories::quest_viewing::QuestViewingRepository;
use crate::domain::value_objects::quest_model::{AddQuestModel, EditQuestModel};

pub struct QuestOpsUseCase<T1, T2>
where
    T1: QuestOpsRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    pub quest_ops_repository: Arc<T1>,
    pub quest_viewing_repository: Arc<T2>,
}

impl<T1, T2> QuestOpsUseCase<T1, T2>
where
    T1: QuestOpsRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    pub fn new(quest_ops_repository: Arc<T1>, quest_viewing_repository: Arc<T2>) -> Self {
        Self { quest_ops_repository, quest_viewing_repository }
    }

    async fn add(&self, add_quest_model: AddQuestModel) -> Result<i32> {
        unimplemented!()
    }
    async fn edit(&self, quest_id: i32, edit_quest_model: EditQuestModel) -> Result<i32> {
        unimplemented!()
    }
    async fn remove(&self, quest_id: i32, guild_commander_id: i32) -> Result<i32> {
        unimplemented!()
    }
}