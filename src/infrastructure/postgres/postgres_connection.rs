﻿use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use anyhow::Result;

pub type PgPoolSquad = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection(database_url: &str) -> Result<PgPoolSquad> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager)?;
    Ok(pool)
}