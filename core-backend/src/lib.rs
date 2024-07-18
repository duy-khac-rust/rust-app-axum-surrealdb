pub mod error;

use std::env::var;

use error::ResultCore;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Web {
    pub addr: String,
}

#[derive(Deserialize)]
pub struct Surrealdb {
    pub addr: String,
    pub username: String,
    pub password: String,
    pub namespace: String,
    pub database: String,
    
}

#[derive(Deserialize)]
pub struct DevConfig {
    pub devweb: Web,
    pub devsurrealdb: Surrealdb,
}

#[derive(Deserialize)]
pub struct ProdConfig {
    pub web: Web,
    pub surrealdb: Surrealdb,
}

#[derive(Deserialize)]
pub struct DevEnv {
    pub app: DevConfig,
}

#[derive(Deserialize)]
pub struct ProdEnv {
    pub app: ProdConfig,
}

impl ProdConfig {
    pub async fn from_env() -> ResultCore<Self> {
        match var("ENV").as_deref() {
            Ok("prod") => {
                let config: ProdEnv = config::Config::builder()
                    .add_source(config::Environment::default())
                    .build()?
                    .try_deserialize::<ProdEnv>()?;
                Ok(Self {
                    web: config.app.web,
                    surrealdb: config.app.surrealdb,
                })
            }

            _ => {
                let config: DevEnv = config::Config::builder()
                    .add_source(config::Environment::default())
                    .build()?
                    .try_deserialize::<DevEnv>()?;
                Ok(Self {
                    web: config.app.devweb,
                    surrealdb: config.app.devsurrealdb,
                })
            }
        }
    }
}
