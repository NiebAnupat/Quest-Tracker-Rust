use crate::domain::entities::adventurers::{AdventurerEntity, RegisterAdventurerEntity};
use crate::domain::repositories::adventurers::AdventurersRepository;
use crate::infrastructure::postgres::postgres_connection::PgPoolSquad;
use crate::infrastructure::postgres::schema::adventurers;
use anyhow::Result;
use async_trait::async_trait;
use diesel::dsl::insert_into;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use std::sync::Arc;
use tracing::info;

pub struct AdventurersPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl AdventurersPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

const SERVICE_CONTEXT: &str = "ADVENTURERS_REPOSITORY";
#[async_trait]
impl AdventurersRepository for AdventurersPostgres {
    async fn register(&self, register_adventurer_entity: RegisterAdventurerEntity) -> Result<i32> {
        const ACTION_NAME: &str = "register";
        info!("START - {}: {} - server_time: {}", SERVICE_CONTEXT, ACTION_NAME, chrono::Utc::now());
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = insert_into(adventurers::table)
            .values(&register_adventurer_entity)
            .returning(adventurers::id)
            .get_result::<i32>(&mut conn)?;

        info!("SUCCESS - {}: {} - payload={:?} - server_time: {}", SERVICE_CONTEXT, ACTION_NAME, result, chrono::Utc::now());
        Ok(result)
    }

    async fn find_by_username(&self, username: String) -> Result<AdventurerEntity> {
        const ACTION_NAME: &str = "find_by_username";
        info!("START - {}: {} - payload={:?} - server_time: {}", SERVICE_CONTEXT, ACTION_NAME, username, chrono::Utc::now());
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = adventurers::table
            .filter(adventurers::username.eq(username))
            .first::<AdventurerEntity>(&mut conn)?;

        info!("SUCCESS - {}: {} - payload={:?} - server_time: {}", SERVICE_CONTEXT, ACTION_NAME, result, chrono::Utc::now());
        Ok(result)
    }
}