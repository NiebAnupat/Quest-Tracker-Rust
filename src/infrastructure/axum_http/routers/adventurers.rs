use crate::application::usecases::adventurers::AdventurersUseCase;
use crate::domain::repositories::adventurers::AdventurersRepository;
use crate::domain::value_objects::adventurer_model::RegisterAdventurerModel;
use crate::infrastructure::postgres::postgres_connection::PgPoolSquad;
use crate::infrastructure::postgres::repositories::adventurers::AdventurersPostgres;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use std::sync::Arc;
use tracing::info;

const SERVICE_CONTEXT: &str = "ADVENTURERS_ROUTER";

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
    const ACTION_NAME: &str = "REGISTER";
    info!("START - {}: {} - server_time: {}", SERVICE_CONTEXT, ACTION_NAME, chrono::Utc::now());

    match adventurers_use_case.register(register_adventurer_model).await {
        Ok(id) => {
            info!("SUCCESS - {}: {} - id={} - server_time: {}", SERVICE_CONTEXT, ACTION_NAME, id, chrono::Utc::now());
            (StatusCode::CREATED, format!("เพิ่มข้อมูลนักผจญภัยสำเร็จ หมายเลขนักผจญภัย: {}", id)).into_response()
        }
        Err(e) => {
            info!("ERROR - {}: {} - error={} - server_time: {}", SERVICE_CONTEXT, ACTION_NAME, e, chrono::Utc::now());
            (StatusCode::INTERNAL_SERVER_ERROR, format!("เพิ่มข้อมูลนักผจญภัยไม่สำเร็จ กรุณาลองใหม่อีกครั้ง : {}", e.to_string())).into_response()
        }
    }
}
