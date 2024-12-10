use crate::application::usecases::quest_viewing::QuestViewingUseCase;
use crate::domain::repositories::quest_viewing::QuestViewingRepository;
use crate::domain::value_objects::board_checking_filter::BoardCheckingFilter;
use crate::infrastructure::postgres::postgres_connection::PgPoolSquad;
use crate::infrastructure::postgres::repositories::quest_viewing::QuestViewingPostgres;
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use std::sync::Arc;

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    // repository -> use case -> router
    let quest_viewing_repository = QuestViewingPostgres::new(db_pool);
    let quest_viewing_use_case = QuestViewingUseCase::new(Arc::new(quest_viewing_repository));

    Router::new()
        .route("/:quest_id", get(view_details))
        .route("/board-checking", get(board_checking))
        .with_state(Arc::new(quest_viewing_use_case))
}

pub async fn view_details<T>(
    State(quest_viewing_use_case): State<Arc<QuestViewingUseCase<T>>>,
    Path(quest_id): Path<i32>)
    -> impl IntoResponse
where
    T: QuestViewingRepository + Send + Sync,
{
    unimplemented!()
}

pub async fn board_checking<T>(
    State(quest_viewing_use_case): State<Arc<QuestViewingUseCase<T>>>,
    filter: Query<BoardCheckingFilter>)
    -> impl IntoResponse
where
    T: QuestViewingRepository + Send + Sync,
{
    unimplemented!()
}
