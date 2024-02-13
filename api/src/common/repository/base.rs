use log::{error, info};
use sqlx::{Pool, Postgres, migrate, FromRow};
use std::env;
use dotenv::dotenv;
use async_trait::async_trait;
use tokio::time::{sleep, Duration};

#[derive(FromRow, Clone)]
pub struct EntityId {
    pub id: i64
}

#[async_trait]
pub trait Repository{
    async fn init() -> Self;
}

#[derive(Debug, Clone)]
pub struct DbRepo {
    conn: Pool<Postgres>
}

#[async_trait]
impl Repository for DbRepo {
    async fn init() -> Self {
        DbRepo {
            conn: get_conn_pool().await
        }
    }
}

pub trait ConnGetter: Repository {
    type Output;

    fn get_conn(&self) -> &Self::Output;
}

impl ConnGetter for DbRepo {
    type Output = Pool<Postgres>;

    fn get_conn(&self) -> &Self::Output {
        &self.conn
    }
}

async fn get_conn_pool() -> Pool<Postgres> {
    dotenv().ok();
    let postgres_host = env::var("POSTGRES_HOST").unwrap();
    let postgres_port = env::var("POSTGRES_PORT").unwrap().parse::<u16>().unwrap();
    let postgres_password = env::var("POSTGRES_PASSWORD").unwrap();
    let postgres_user = env::var("POSTGRES_USER").unwrap();
    let postgres_db = env::var("POSTGRES_DB").unwrap();

    let postgres_url = format!(
        "postgres://{postgres_user}:{postgres_password}@{postgres_host}:{postgres_port}/{postgres_db}"
    );
    
    let mut retry_limit = 0;
    let mut conn: Option<Pool<Postgres>> = None;
    loop {
        if retry_limit > 3 {
            panic!("Attempts to connect to db have failed, exiting ...");
        }
        let conn_result = sqlx::postgres::PgPool::connect(&postgres_url).await;
        match conn_result {
            Ok(some_conn) => {
                let migrate_result = migrate!("./migrations").run(&some_conn).await;
                match migrate_result {
                    Ok(()) => {
                        info!("migration complete");
                    },
                    Err(e) => error!("failed to migrate {}", e)
                };
                conn = Some(some_conn);
                break;
            },
            Err(ref e) => {
                error!("Failed to connect to db: {}", e);
                continue;
            }
        }
        info!("Failed to connect to db, trying again after 5 seconds ...");
        retry_limit += 1;
        sleep(Duration::from_secs(5)).await;
    }

    conn.unwrap()
}