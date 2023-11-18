use crate::db::schema::posts;
use crate::db::DbSqlx;
use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use serde::Deserialize;
use serde::Serialize;
use std::ops::DerefMut;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = posts)]
#[derive(sqlx::FromRow)]
pub struct Post {
    pub id: Option<i64>,
    pub title: String,
    pub text: String,
    pub published: bool,
}

type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

#[post("/create", data = "<post>")]
pub async fn create(mut db: Connection<DbSqlx>, post: Json<Post>) -> Result<Json<Post>> {
    let post_new = sqlx::query_as::<_, Post>(
        r#"
        INSERT INTO posts (title, text, published) 
        VALUES ($1, $2, $3)
        RETURNING id, title, text, published
        "#,
    )
    .bind(post.title.to_owned())
    .bind(post.text.to_owned())
    .bind(post.published)
    .fetch_one(&mut **db)
    .await?;

    debug!("post_new: {:?}", post_new);

    Ok(Json(post_new))
}

#[post("/create2", data = "<post>")]
pub async fn create2(mut db: Connection<DbSqlx>, post: Json<Post>) -> Result<Json<Post>> {
    let post_new = sqlx::query_as!(
        Post,
        r#"
        INSERT INTO posts (title, text, published) 
        VALUES ($1, $2, $3)
        RETURNING id, title, text, published
        "#,
        post.title,
        post.text,
        post.published
    )
    .fetch_one(&mut **db)
    .await?;

    debug!("post_new: {:?}", post_new);

    Ok(Json(post_new))
}

#[get("/<id>")]
pub async fn find(id: &str, mut db: Connection<DbSqlx>) -> Result<Json<Option<Post>>> {
    let find_post = sqlx::query_as!(
        Post,
        r#"
        SELECT id, title, text, published
        FROM posts
        WHERE id = $1
        "#,
        id.parse::<i64>().unwrap()
    )
    .fetch_optional(&mut **db)
    .await?;

    debug!("find_post: {:?}", find_post);

    Ok(Json(find_post))
}

#[get("/find_all")]
pub async fn find_all(mut db: Connection<DbSqlx>) -> Result<Json<Vec<Post>>> {
    let r = db.deref_mut().deref_mut();

    let find_post = sqlx::query_as!(
        Post,
        r#"
        SELECT *
        FROM posts
        "#,
    )
    .fetch_all(r)
    .await?;

    debug!("find_post: {:?}", find_post);

    Ok(Json(find_post))
}
