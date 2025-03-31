use actix_web::{web, HttpResponse, Error};
use sqlx::PgPool;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;

use crate::models::{NewUser, LoginCredentials, User, Content, NewContent, UpdateContent};

pub async fn register(
    pool: web::Data<PgPool>,
    user: web::Json<NewUser>,
) -> Result<HttpResponse, Error> {
    let hashed_password = hash(user.password.as_bytes(), DEFAULT_COST)
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, password_hash, created_at)
        VALUES ($1, $2, $3)
        RETURNING id, username, password_hash, created_at
        "#,
        user.username,
        hashed_password,
        Utc::now().naive_utc()
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|_| HttpResponse::InternalServerError().finish())?;

    Ok(HttpResponse::Created().json(user))
}

pub async fn login(
    pool: web::Data<PgPool>,
    creds: web::Json<LoginCredentials>,
) -> Result<HttpResponse, Error> {
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE username = $1",
        creds.username
    )
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|_| HttpResponse::InternalServerError().finish())?;

    match user {
        Some(user) => {
            if verify(&creds.password, &user.password_hash)
                .map_err(|_| HttpResponse::InternalServerError().finish())? {
                Ok(HttpResponse::Ok().json(user))
            } else {
                Ok(HttpResponse::Unauthorized().finish())
            }
        }
        None => Ok(HttpResponse::Unauthorized().finish()),
    }
}

pub async fn create_content(
    pool: web::Data<PgPool>,
    content: web::Json<NewContent>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let now = Utc::now().naive_utc();
    
    let content = sqlx::query_as!(
        Content,
        r#"
        INSERT INTO contents (title, body, author_id, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, title, body, author_id, created_at, updated_at
        "#,
        content.title,
        content.body,
        user_id.into_inner(),
        now,
        now
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|_| HttpResponse::InternalServerError().finish())?;

    Ok(HttpResponse::Created().json(content))
}

pub async fn get_content(
    pool: web::Data<PgPool>,
    content_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let content = sqlx::query_as!(
        Content,
        "SELECT * FROM contents WHERE id = $1",
        content_id.into_inner()
    )
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|_| HttpResponse::InternalServerError().finish())?;

    match content {
        Some(content) => Ok(HttpResponse::Ok().json(content)),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

pub async fn update_content(
    pool: web::Data<PgPool>,
    content_id: web::Path<i32>,
    update: web::Json<UpdateContent>,
) -> Result<HttpResponse, Error> {
    let content = sqlx::query_as!(
        Content,
        r#"
        UPDATE contents
        SET title = COALESCE($1, title),
            body = COALESCE($2, body),
            updated_at = $3
        WHERE id = $4
        RETURNING id, title, body, author_id, created_at, updated_at
        "#,
        update.title,
        update.body,
        Utc::now().naive_utc(),
        content_id.into_inner()
    )
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|_| HttpResponse::InternalServerError().finish())?;

    match content {
        Some(content) => Ok(HttpResponse::Ok().json(content)),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

pub async fn delete_content(
    pool: web::Data<PgPool>,
    content_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let result = sqlx::query!(
        "DELETE FROM contents WHERE id = $1",
        content_id.into_inner()
    )
    .execute(pool.get_ref())
    .await
    .map_err(|_| HttpResponse::InternalServerError().finish())?;

    if result.rows_affected() > 0 {
        Ok(HttpResponse::NoContent().finish())
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}