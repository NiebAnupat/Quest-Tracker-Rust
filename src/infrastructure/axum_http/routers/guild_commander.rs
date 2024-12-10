use std::sync::Arc;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{Json, Router};
use axum::routing::post;
use crate::application::usecases::guild_commander::GuildCommanderUseCase;
use crate::domain::repositories::guild_commander::GuildCommanderRepository;
use crate::domain::value_objects::guild_commander_model::RegisterGuildCommanderModel;
use crate::infrastructure::postgres::postgres_connection::PgPoolSquad;
use crate::infrastructure::postgres::repositories::guild_commander::GuildCommanderPostgres;

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    // repository -> use case -> router
    let guild_commander_repository = GuildCommanderPostgres::new(db_pool);
    let guild_commander_use_case = GuildCommanderUseCase::new(Arc::new(guild_commander_repository));

    Router::new()
        .route("/", post(register))
        .with_state(Arc::new(guild_commander_use_case))
}

pub async fn register<T>(
    State(guild_commander_use_case): State<Arc<GuildCommanderUseCase<T>>>
    , Json(register_guild_commander_model): Json<RegisterGuildCommanderModel>)
    -> impl IntoResponse
where
    T: GuildCommanderRepository + Send + Sync,
{
    unimplemented!()
}
