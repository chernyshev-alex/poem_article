use crate::config;
use std::time::Duration;
use stdx::log::error;
use stdx::sqlx::{self, postgres::PgPoolOptions, Executor, Pool, Postgres, Transaction};

pub type DB = Pool<Postgres>;
pub trait Queryer<'c>: Executor<'c, Database = sqlx::Postgres> {}

impl<'c> Queryer<'c> for &Pool<Postgres> {}
impl<'c> Queryer<'c> for &'c mut Transaction<'_, Postgres> {}

pub async fn connect(database: &config::Database) -> Result<DB, crate::Error> {
    PgPoolOptions::new()
        .max_connections(database.pool_size)
        .max_lifetime(Duration::from_secs(30 * 60))
        .connect(&database.url)
        .await?
        .expect("db connection faild")
}

pub async fn migrate(db: &DB) -> Result<(), crate::Error> {
    match sqlx::migrate!("../db/migrations").run(db).await {
        Ok(_) => Ok(()),
        Err(err) => {
            error!("kernel.migrate: migrating: {}", &err);
            Err(err)
        }
    }?;
    Ok(())
}
