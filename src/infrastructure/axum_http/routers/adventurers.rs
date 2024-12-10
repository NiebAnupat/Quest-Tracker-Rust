use std::sync::Arc;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{Json, Router};
use axum::routing::post;
use crate::application::usecases::adventurers::AdventurersUseCase;
use crate::domain::repositories::adventurers::AdventurersRepository;
use crate::domain::value_objects::adventurer_model::RegisterAdventurerModel;
use crate::infrastructure::postgres::postgres_connection::PgPoolSquad;
use crate::infrastructure::postgres::repositories::adventurers::AdventurersPostgres;

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    // repository -> use case -> router
    let adventurers_repository = AdventurersPostgres::new(db_pool);
    let adventurers_use_case = AdventurersUseCase::new(Arc::new(adventurers_repository));

    Router::new()
        .route("/", post(register))
        .with_state(Arc::new(adventurers_use_case))
}

pub async fn register<T>(
    State(adventurers_use_case): State<Arc<AdventurersUseCase<T>>>
    , Json(register_adventurer_model): Json<RegisterAdventurerModel>)
    -> impl IntoResponse
where
    T: AdventurersRepository + Send + Sync,
{
    unimplemented!()
}
