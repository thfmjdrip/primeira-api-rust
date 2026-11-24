use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub async fn start_pg() -> Pool<Postgres> {
    let postgres_env = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&postgres_env)
        .await
        .expect("Failed to connect");
    let check_migrate = sqlx::migrate!("src/databases/migrations/").run(&pool).await;
    match check_migrate {
        Ok(sucesso) => {
            println!("Migrations ran sucessfuly {:?}", sucesso)
        }
        Err(falha) => {
            println!("Migrations ran with error  {:?}", falha)
        }
    }
    pool
}
