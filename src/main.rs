use poem::listener::TcpListener;
use poem::Server;
use poem_article::repositories::DbRepository;
use poem_article::services::{ArticleServiceSt, SocialMediaPublisher};
use poem_article::{handlers, AppConfig, AppStateM};
use sea_orm::Database;
use tera::Tera;

use crate::handlers::*;
use std::sync::Arc;
mod migration;

// TODO : open API

#[tokio::main]
async fn start() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    //  TODO  :  let orm_metrics = sea_orm::metric::Info();

    let conf = AppConfig::load()?;
    // migration::Migrator::up(conn, None).await.unwrap();

    let conn = Database::connect(conf.db_url).await.unwrap();
    let repo = DbRepository::new(Arc::new(conn));
    let service = ArticleServiceSt::new(Arc::new(repo));
    let app_state = AppStateM {
        service: Arc::new(service),
        publisher: Arc::new(SocialMediaPublisher {}),
        templates: Tera::new("./templates/**/*").unwrap(),
    };
    println!("{}:{}", conf.host, conf.port);
    Server::new(TcpListener::bind(format!("{}:{}", conf.host, conf.port)))
        .run(config_router(app_state))
        .await
}

fn main() {
    if let Some(err) = start().err() {
        println!("app error {err}")
    }
}
