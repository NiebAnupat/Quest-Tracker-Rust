use crate::application::usecases::journey_ledger::JourneyLedgerUseCase;
use crate::domain::repositories::journey_ledger::JourneyLedgerRepository;
use crate::domain::repositories::quest_viewing::QuestViewingRepository;
use crate::infrastructure::postgres::postgres_connection::PgPoolSquad;
use crate::infrastructure::postgres::repositories::journey_ledger::JourneyLedgerPostgres;
use crate::infrastructure::postgres::repositories::quest_viewing::QuestViewingPostgres;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::patch;
use axum::{Extension, Router};
use std::sync::Arc;

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    // repository -> use case -> router
    let journey_ledger_repository = JourneyLedgerPostgres::new(Arc::clone(&db_pool));
    let quest_viewing_repository = QuestViewingPostgres::new(Arc::clone(&db_pool));
    let journey_ledger_use_case = JourneyLedgerUseCase::new(Arc::new(journey_ledger_repository), Arc::new(quest_viewing_repository));

    Router::new()
        .route("/in-journey/:quest_id", patch(in_journey))
        .route("/to-completed/:quest_id", patch(to_completed))
        .route("/to-failed/:quest_id", patch(to_failed))
        .with_state(Arc::new(journey_ledger_use_case))
}

async fn in_journey<T1, T2>(
    State(journey_ledger_use_case): State<Arc<JourneyLedgerUseCase<T1, T2>>>,
    Path(quest_id): Path<i32>,
    Extension(guild_commander_id): Extension<i32>,
)
    -> impl IntoResponse
where
    T1: JourneyLedgerRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    unimplemented!()
}
async fn to_completed<T1, T2>(
    State(journey_ledger_use_case): State<Arc<JourneyLedgerUseCase<T1, T2>>>,
    Path(quest_id): Path<i32>,
    Extension(guild_commander_id): Extension<i32>,
)
    -> impl IntoResponse
where
    T1: JourneyLedgerRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    unimplemented!()
}
async fn to_failed<T1, T2>(
    State(journey_ledger_use_case): State<Arc<JourneyLedgerUseCase<T1, T2>>>,
    Path(quest_id): Path<i32>,
    Extension(guild_commander_id): Extension<i32>,
)
    -> impl IntoResponse
where
    T1: JourneyLedgerRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    unimplemented!()
}