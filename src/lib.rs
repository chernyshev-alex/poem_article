pub mod domain;
pub mod handlers;
pub mod repositories;
pub mod services;

use crate::services::{ArticleServiceTrait, SocialMediaPublisherTrait};
use std::{env, sync::Arc};

#[derive(Debug, Clone)]
pub struct AppState {
    pub templates: Option<tera::Tera>,
}
#[derive(Debug, Clone)]
pub struct AppStateM {
    pub service: Arc<dyn ArticleServiceTrait>,
    pub publisher: Arc<dyn SocialMediaPublisherTrait>,
    pub templates: tera::Tera,
}

pub struct AppConfig {
    pub host: String,
    pub port: String,
    pub db_url: String,
}
impl AppConfig {
    pub fn load() -> Result<AppConfig, std::io::Error> {
        dotenvy::dotenv().ok();
        Ok(AppConfig {
            host: env::var("HOST").unwrap_or("0.0.0.1".to_string()),
            port: env::var("PORT").unwrap_or("8000".to_string()),
            db_url: env::var("DATABASE_URL").expect("db url is expected"),
        })
    }
}
