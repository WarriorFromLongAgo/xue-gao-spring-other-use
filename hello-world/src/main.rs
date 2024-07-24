use std::{env, io};

use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, web};
use chrono::NaiveDateTime;
use log::info;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Postgres};
use sqlx::postgres::PgPoolOptions;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
    pub email: String,
    // 使用 chrono 的 NaiveDateTime 来映射 TIMESTAMP WITHOUT TIME ZONE
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/db_get")]
async fn db_get(pool: web::Data<PgPool>) -> impl Responder {
    let user_v2 = sqlx::query_as::<Postgres, UserInfo>("SELECT id, username, email, created_at, updated_at FROM user_info where username = $1")
        .bind("user1")
        .fetch_one(pool.get_ref())
        .await;
    match user_v2 {
        // 查询成功，返回用户信息
        Ok(user_v2) => HttpResponse::Ok().json(user_v2),
        // 查询失败，返回内部服务器错误
        Err(e) => {
            eprintln!("Database query failed: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/db_list")]
async fn db_list(pool: web::Data<PgPool>) -> impl Responder {

    // 直接使用 fetch_all 方法并指定返回类型为 Vec<UserInfo>
    let users = sqlx::query_as::<Postgres, UserInfo>("SELECT id, username, email, created_at, updated_at FROM user_info")
        .fetch_all(pool.get_ref())
        .await;
    match users {
        // 查询成功，返回用户信息
        Ok(user_v2) => HttpResponse::Ok().json(user_v2),
        // 查询失败，返回内部服务器错误
        Err(e) => {
            eprintln!("Database query failed: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

fn init_logger() {
    use std::io::Write;

    let env = env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
    // env::set_var("RUST_LOG", "sqlx=debug");
    env::set_var("RUST_LOG", "debug");
    // 设置日志打印格式
    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] {}",
                record.level(),
                record.module_path().unwrap_or("<unnamed>"),
                &record.args()
            )
        })
        .init();
    info!("env_logger initialized.");
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let url = "postgres://postgres:123456@192.168.23.130:5432/chain_scan";

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(url)
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?; // 转换错误

    init_logger();


    HttpServer::new(move|| {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(hello)
            .service(db_get)
            .service(db_list)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind(("127.0.0.1", 9000))?
        .run()
        .await
}