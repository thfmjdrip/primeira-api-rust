use super::models::{AllUsers, Register};
use crate::AppState;
use actix_web::{HttpResponse, Responder, get, post, put, web};
use bcrypt::{hash, DEFAULT_COST};

#[get("/users")]
pub async fn get_all_users(app_state: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query_as::<_, AllUsers>("SELECT id, name, email, password FROM users")
        .fetch_all(&app_state.postgres_client)
        .await;

    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().body("Error trying to get all users from db"),
    }
}

#[put("/users/{id}")]
async fn create_users(
    app_state: web::Data<AppState>,
    user: web::Json<Register>,
    id:web::Path<i32>
) -> impl Responder {
    let register = user.into_inner();

    let hashed = match hash(&register.password, DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(_) => {
            return HttpResponse::InternalServerError().body("Error trying to hash password");
        }
    };

    let result = sqlx::query_as::<_, AllUsers>(
        "UPDATE users SET name = 1$, email= &2, password = 3$  WHERE id = $4 RETURNING id, name, email, password",
    )
        .bind(register.name)
        .bind(register.email)
        .bind(hashed)
        .bind(id.into_inner())
        .fetch_one(&app_state.postgres_client)
        .await;

    match result {
        Ok(created) => HttpResponse::Created().json(created),
        Err(_) => HttpResponse::InternalServerError().body("Error trying to create user"),
    }
}


#[post("/users")]
async fn update_users(
    app_state: web::Data<AppState>,
    user: web::Json<Register>,
) -> impl Responder {
    let register = user.into_inner();

    let hashed = match hash(&register.password, DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(_) => {
            return HttpResponse::InternalServerError().body("Error trying to hash password");
        }
    };

    let result = sqlx::query_as::<_, AllUsers>(
        "INSERT INTO users (name, email, password) VALUES ($1, $2, $3) RETURNING id, name, email, password",
    )
    .bind(register.name)
    .bind(register.email)
    .bind(hashed)
    .fetch_one(&app_state.postgres_client)
    .await;

    match result {
        Ok(created) => HttpResponse::Created().json(created),
        Err(_) => HttpResponse::InternalServerError().body("Error trying to create user"),
    }
}

pub fn users_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_users);
    cfg.service(create_users);
    cfg.service(update_users);
}
