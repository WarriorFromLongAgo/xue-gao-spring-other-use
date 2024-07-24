use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{Arguments, FromRow, Row};
use sqlx::postgres::{PgArguments, PgPoolOptions};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // mysql://username:password@hostname:port/database
    // postgres://username:password@hostname:port/database

    let url = "postgres://postgres:123456@192.168.23.130:5432/chain_scan";

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(url)
        .await?;

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool).await?;

    eprintln!(" {:?} ", row);

    assert_eq!(row.0, 150);

    Ok(())
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
    pub email: String,
    // 使用 chrono 的 NaiveDateTime 来映射 TIMESTAMP WITHOUT TIME ZONE
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[tokio::test]
async fn test_query1_list_user() -> Result<(), sqlx::Error> {
    let url = "postgres://postgres:123456@192.168.23.130:5432/chain_scan";

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(url)
        .await?;

    let rows = sqlx::query("SELECT id, username, email, created_at, updated_at FROM user_info")
        .fetch_all(&pool)
        .await?;

    let users: Vec<UserInfo> = rows.into_iter().map(|row| {
        UserInfo {
            id: row.get(0),
            username: row.get(1),
            email: row.get(2),
            created_at: row.get(3),
            updated_at: row.get(4),
        }
    }).collect();

    for user in users {
        println!("User: {:?}", user);
    }

    // User: UserInfo { id: 1, username: "user1", email: "user1", created_at: 2024-07-24T17:27:32.234231, updated_at: 2024-07-24T17:27:32.234231 }

    // let rows = sqlx::query!("SELECT id, username, email, created_at, updated_at FROM user_info")
    //     .fetch_all(&pool)
    //     .await?;

    Ok(())
}

#[tokio::test]
async fn test_query_as_user_info_v1() -> Result<(), sqlx::Error> {
    let url = "postgres://postgres:123456@192.168.23.130:5432/chain_scan";
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(url)
        .await?;


    // 直接使用 fetch_all 方法并指定返回类型为 Vec<UserInfo>
    let users: Vec<UserInfo> = sqlx::query_as("SELECT id, username, email, created_at, updated_at FROM user_info")
        .fetch_all(&pool)
        .await?;

    for user in users {
        println!("User: {:?}", user);
    }

    // User: UserInfo { id: 1, username: "user1", email: "user1", created_at: 2024-07-24T17:27:32.234231, updated_at: 2024-07-24T17:27:32.234231 }

    Ok(())
}

#[tokio::test]
async fn test_query_user_info_v2() -> Result<(), sqlx::Error> {
    let url = "postgres://postgres:123456@192.168.23.130:5432/chain_scan";
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(url)
        .await?;


    // 直接使用 fetch_all 方法并指定返回类型为 Vec<UserInfo>
    let users: Vec<UserInfo> = sqlx::query_as("SELECT id, username, email, created_at, updated_at FROM user_info where username = $1")
        .bind("user1")
        .fetch_all(&pool)
        .await?;

    for user in users {
        eprintln!("User: {:?}", user);
    }

    // User: UserInfo { id: 1, username: "user1", email: "user1", created_at: 2024-07-24T17:27:32.234231, updated_at: 2024-07-24T17:27:32.234231 }

    eprintln!("===============================================================");

    let user_v2: UserInfo = sqlx::query_as("SELECT id, username, email, created_at, updated_at FROM user_info where username = $1")
        .bind("user1")
        .fetch_one(&pool)
        .await?;

    eprintln!("user_v2: {:?}", user_v2);

    eprintln!("===============================================================");
    let user_v3 = sqlx::query_as::<_, UserInfo>("SELECT id, username, email, created_at, updated_at FROM user_info where username = $1")
        .bind("user1")
        .fetch_one(&pool)
        .await?;


    eprintln!("user_v3: {:?}", user_v3);

    Ok(())
}


#[tokio::test]
async fn test_update_user_info_v1() -> Result<(), sqlx::Error> {
    let url = "postgres://postgres:123456@192.168.23.130:5432/chain_scan";
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(url)
        .await?;

    // 直接使用 fetch_all 方法并指定返回类型为 Vec<UserInfo>
    let users: Vec<UserInfo> = sqlx::query_as("SELECT id, username, email, created_at, updated_at FROM user_info")
        .fetch_all(&pool)
        .await?;
    for user in users {
        eprintln!("User: {:?}", user);
    }

    Ok(())
}


#[tokio::test]
async fn test_insert_user_info_v1() -> Result<(), sqlx::Error> {
    let url = "postgres://postgres:123456@192.168.23.130:5432/chain_scan";
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(url)
        .await?;

    let users = vec![
        ("user_100", "user_100@example.com"),
    ];

    // 动态构建批量插入的 SQL 语句
    let mut query_str = String::from("INSERT INTO user_info (username, email) VALUES ");
    let mut query_values = PgArguments::default();
    for (i, (username, email)) in users.iter().enumerate() {
        if i > 0 {
            query_str.push_str(", ");
        }
        query_str.push_str(&format!("(${}, ${})", i * 2 + 1, i * 2 + 2));
        query_values.add(username);
        query_values.add(email);
    }
    query_str.push_str(" RETURNING *");

    eprintln!("sql {}", query_str);

    // 使用 query_as 来执行动态构建的 SQL 语句
    let inserted_users: Vec<UserInfo> = sqlx::query_as_with(&query_str, query_values)
        .fetch_all(&pool)
        .await?;

    for user in inserted_users {
        eprintln!("{:?}", user);
    }

    Ok(())
}


#[tokio::test]
async fn test_batch_insert_user_info_v1() -> Result<(), sqlx::Error> {
    let url = "postgres://postgres:123456@192.168.23.130:5432/chain_scan";
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(url)
        .await?;

    let users = vec![
        ("user_200", "user_200@example.com"),
        ("user_201", "user_201@example.com"),
    ];

    // 动态构建批量插入的 SQL 语句
    let mut query_str = String::from("INSERT INTO user_info (username, email) VALUES ");
    let mut query_values = PgArguments::default();
    for (i, (username, email)) in users.iter().enumerate() {
        if i > 0 {
            query_str.push_str(", ");
        }
        query_str.push_str(&format!("(${}, ${})", i * 2 + 1, i * 2 + 2));
        query_values.add(username);
        query_values.add(email);
    }
    query_str.push_str(" RETURNING *");

    eprintln!("sql {}", query_str);

    // 使用 query_as 来执行动态构建的 SQL 语句
    let inserted_users: Vec<UserInfo> = sqlx::query_as_with(&query_str, query_values)
        .fetch_all(&pool)
        .await?;

    for user in inserted_users {
        eprintln!("{:?}", user);
    }

    Ok(())
}
