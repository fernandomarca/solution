use rocket_db_pools::diesel::PgPool;
use rocket_db_pools::sqlx;
use rocket_db_pools::Database;

pub mod schema;

#[derive(Database)]
#[database("diesel_postgres")]
pub struct Db(PgPool);

#[derive(Database)]
#[database("sqlx")]
pub struct DbSqlx(sqlx::PgPool);
