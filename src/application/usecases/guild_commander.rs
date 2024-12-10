use std::sync::Arc;
use anyhow::Result;
use crate::domain::repositories::guild_commander::GuildCommanderRepository;
use crate::domain::value_objects::guild_commander_model::RegisterGuildCommanderModel;

pub struct GuildCommanderUseCase<T>
where
    T: GuildCommanderRepository + Send + Sync,
{
    pub guild_commander_repository: Arc<T>,
}

impl<T> GuildCommanderUseCase<T>
where
    T: GuildCommanderRepository + Send + Sync,
{
    pub fn new(guild_commander_repository: Arc<T>) -> Self {
        Self { guild_commander_repository }
    }

    pub async fn register(&self, mut register_guild_commander_model: RegisterGuildCommanderModel) -> Result<i32> {
        unimplemented!()
    }
}
