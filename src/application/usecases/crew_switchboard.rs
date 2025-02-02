﻿use crate::domain::repositories::crew_switchboard::CrewSwitchboardRepository;
use crate::domain::repositories::quest_viewing::QuestViewingRepository;
use anyhow::Result;
use std::sync::Arc;

pub struct CrewSwitchboardUseCase<T1, T2>
where
    T1: CrewSwitchboardRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    pub crew_switchboard_repository: Arc<T1>,
    pub quest_viewing_repository: Arc<T2>,
}

impl<T1, T2> CrewSwitchboardUseCase<T1, T2>
where
    T1: CrewSwitchboardRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    pub fn new(crew_switchboard_repository: Arc<T1>, quest_viewing_repository: Arc<T2>) -> Self {
        Self { crew_switchboard_repository, quest_viewing_repository }
    }

    pub async fn join(&self, quest_id: i32, adventurer_id: i32) -> Result<()> {
        unimplemented!()
    }

    pub async fn leave(&self, quest_id: i32, adventurer_id: i32) -> Result<()> {
        unimplemented!()
    }
}