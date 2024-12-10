use std::sync::Arc;

use axum::{
    extract::State,
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use cookie::time::Duration;

use crate::{
    application::usecases::authentication::AuthenticationUseCase,
    config::{config_loader::get_stage, stage::Stage},
};
use crate::domain::repositories::adventurers::AdventurersRepository;
use crate::domain::repositories::guild_commander::GuildCommanderRepository;
use crate::infrastructure::postgres::postgres_connection::PgPoolSquad;
use crate::infrastructure::postgres::repositories::adventurers::AdventurersPostgres;
use crate::infrastructure::postgres::repositories::guild_commander::GuildCommanderPostgres;

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let adventurers_repository = AdventurersPostgres::new(Arc::clone(&db_pool));
    let guild_commander_repository = GuildCommanderPostgres::new(Arc::clone(&db_pool));
    let authentication_use_case = AuthenticationUseCase::new(
        Arc::new(adventurers_repository),
        Arc::new(guild_commander_repository),
    );

    Router::new()
        .route("/adventurers/login", post(adventurers_login))
        .route(
            "/adventurers/refresh-token",
            post(adventurers_refresh_token),
        )
        .route("/guild-commanders/login", post(guild_commanders_login))
        .route(
            "/guild-commanders/refresh-token",
            post(guild_commanders_refresh_token),
        )
        .with_state(Arc::new(authentication_use_case))
}

pub async fn adventurers_login<T1, T2>(
    State(authentication_use_case): State<Arc<AuthenticationUseCase<T1, T2>>>,
) -> impl IntoResponse
where
    T1: AdventurersRepository + Send + Sync,
    T2: GuildCommanderRepository + Send + Sync,
{
    unimplemented!()
}

pub async fn adventurers_refresh_token<T1, T2>(
    State(authentication_use_case): State<Arc<AuthenticationUseCase<T1, T2>>>,
) -> impl IntoResponse
where
    T1: AdventurersRepository + Send + Sync,
    T2: GuildCommanderRepository + Send + Sync,
{
    unimplemented!()
}

pub async fn guild_commanders_login<T1, T2>(
    State(authentication_use_case): State<Arc<AuthenticationUseCase<T1, T2>>>,
) -> impl IntoResponse
where
    T1: AdventurersRepository + Send + Sync,
    T2: GuildCommanderRepository + Send + Sync,
{
    unimplemented!()
}

pub async fn guild_commanders_refresh_token<T1, T2>(
    State(authentication_use_case): State<Arc<AuthenticationUseCase<T1, T2>>>,
) -> impl IntoResponse
where
    T1: AdventurersRepository + Send + Sync,
    T2: GuildCommanderRepository + Send + Sync,
{
    unimplemented!()
}