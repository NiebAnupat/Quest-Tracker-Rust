use crate::application::usecases::crew_switchboard::CrewSwitchboardUseCase;
use crate::domain::repositories::crew_switchboard::CrewSwitchboardRepository;
use crate::domain::repositories::quest_viewing::QuestViewingRepository;
use crate::infrastructure::postgres::postgres_connection::PgPoolSquad;
use crate::infrastructure::postgres::repositories::crew_switchboard::CrewSwitchboardPostgres;
use crate::infrastructure::postgres::repositories::quest_viewing::QuestViewingPostgres;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{delete, post};
use axum::{Extension, Router};
use std::sync::Arc;

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    // repository -> use case -> router
    let crew_switchboard_repository = CrewSwitchboardPostgres::new(Arc::clone(&db_pool));
    let quest_viewing_repository = QuestViewingPostgres::new(Arc::clone(&db_pool));
    let crew_switchboard_use_case = CrewSwitchboardUseCase::new(Arc::new(crew_switchboard_repository), Arc::new(quest_viewing_repository));

    Router::new()
        .route("/join/:quest_id", post(join))
        .route("/leave/:quest_id", delete(leave))
        .with_state(Arc::new(crew_switchboard_use_case))
}

pub async fn join<T1, T2>(
    State(crew_switchboard_use_case): State<Arc<CrewSwitchboardUseCase<T1, T2>>>,
    Path(quest_id): Path<i32>,
    Extension(adventurer_id): Extension<i32>)
    -> impl IntoResponse
where
    T1: CrewSwitchboardRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    unimplemented!()
}

pub async fn leave<T1, T2>(
    State(crew_switchboard_use_case): State<Arc<CrewSwitchboardUseCase<T1, T2>>>,
    Path(quest_id): Path<i32>,
    Extension(adventurer_id): Extension<i32>)
    -> impl IntoResponse
where
    T1: CrewSwitchboardRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    unimplemented!()
}

