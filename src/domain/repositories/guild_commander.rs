use crate::domain::entities::guild_commanders::GuildCommanderEntity;
use crate::domain::value_objects::guild_commander_model::RegisterGuildCommanderModel;
use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;

#[async_trait]
#[automock]
pub trait GuildCommanderRepository {
    async fn register(&self, register_guild_commander_model: RegisterGuildCommanderModel) -> Result<i32>;
    async fn find_by_username(&self, username: String) -> Result<GuildCommanderEntity>;
}