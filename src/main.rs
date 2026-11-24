use crate::databases::start_pg;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use sqlx::{Pool, Postgres};
mod databases;
mod services;
use dotenv::dotenv;

#[derive(Debug, Clone)]
pub struct AppState {
    postgres_client: Pool<Postgres>,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

#[actix_web::main()]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let _pool = start_pg().await;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                postgres_client: _pool.clone(),
            }))
            .service(index)
            .configure(services::users::service::users_routes)
    })
    .bind(("127.0.0.1", 8087))?
    .run()
    .await
}
