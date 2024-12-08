use std::fmt;
use anyhow::Result;

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Stage {
    Local,
    #[default]
    Development,
    Production,
}

impl fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stage::Local => write!(f, "local"),
            Stage::Development => write!(f, "development"),
            Stage::Production => write!(f, "production"),
        }
    }
}

impl Stage {
    pub fn try_from(stage: &str) -> Result<Self> {
        match stage {
            "local" => Ok(Stage::Local),
            "development" => Ok(Stage::Development),
            "production" => Ok(Stage::Production),
            _ => Err(anyhow::anyhow!("Invalid stage")),
        }
    }
}