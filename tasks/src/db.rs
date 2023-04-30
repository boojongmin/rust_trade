use anyhow::Result;
use std::{env};


use chrono::Utc;
use sqlx::{postgres::PgPoolOptions, PgPool, Postgres, PgConnection, Connection, types::chrono::{NaiveDateTime, DateTime, Local}};



impl juniper::Context for DB {}

#[derive(Debug)]
pub struct DB {
    pool: PgPool,
}

pub async fn create_db(max: u32) -> anyhow::Result<DB>  {
    let pool = PgPoolOptions::new()
        .max_connections(max)
        .connect(&env::var("DATABASE_URL")?)
        .await?;
    Ok(DB { pool })
}


