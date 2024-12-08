use anyhow::Result;
use super::config_model::{DotEnvyConfig, Server, Database, AdventurerSecret, GuildCommanderSecret};
use std::env::var as env_var;
use crate::config::stage::Stage;

pub fn load() -> Result<DotEnvyConfig> {
    dotenvy::dotenv().ok();

    let server = Server {
        port: env_var("SERVER_PORT").expect("SERVER_PORT is invalid").parse()?,
        body_limit: env_var("SERVER_BODY_LIMIT").expect("SERVER_BODY_LIMIT is invalid").parse()?,
        timeout: env_var("SERVER_TIMEOUT").expect("SERVER_TIMEOUT is invalid").parse()?,
    };

    let database = Database {
        url: env_var("DATABASE_URL").expect("DATABASE_URL is invalid"),
    };

    Ok(DotEnvyConfig {
        server,
        database,
    })
}

pub fn get_stage() -> Stage {
    dotenvy::dotenv().ok();
    let stage_str = env_var("STAGE").unwrap_or_default();
    Stage::try_from(&stage_str).unwrap_or_default()
}

pub fn get_adventurer_secret() -> Result<AdventurerSecret> {
    dotenvy::dotenv().ok();
    Ok(AdventurerSecret {
        secret: env_var("JWT_ADVENTURER_SECRET").expect("ADVENTURER_SECRET is invalid"),
        refresh_secret: env_var("JWT_ADVENTURER_REFRESH_SECRET").expect("ADVENTURER_REFRESH_SECRET is invalid"),
    })
}

pub fn get_guild_commander_secret() -> Result<GuildCommanderSecret> {
    dotenvy::dotenv().ok();
    Ok(GuildCommanderSecret {
        secret: env_var("JWT_GUILD_COMMANDER_SECRET").expect("GUILD_COMMANDER_SECRET is invalid"),
        refresh_secret: env_var("JWT_GUILD_COMMANDER_REFRESH_SECRET").expect("GUILD_COMMANDER_REFRESH_SECRET is invalid"),
    })
}