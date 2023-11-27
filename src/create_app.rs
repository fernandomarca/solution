use crate::infra::db::DbSqlx;
use crate::middler::RemoveServerHeader;
use dotenvy::dotenv;
use rocket::http::Status;
use serde_json::Value;

use crate::infra::adapters::rabbitmq_adapter::RabbitMqAdapter;
use rocket::catch;
use rocket::catchers;
use rocket::get;
use rocket::routes;
use rocket::Build;
use rocket::Request;
use rocket::Rocket;
use rocket_async_compression::Compression;
use rocket_db_pools::Config;
use rocket_db_pools::Database;
use rocket_dyn_templates::Template;

#[get("/")]
fn index() -> &'static str {
    "root"
}
pub fn start_app() -> Rocket<Build> {
    dotenv().expect(".env file not found");
    let figment = rocket::Config::figment().merge((
        "databases.sqlx",
        Config {
            max_connections: 20,
            connect_timeout: 1,
            idle_timeout: None,
            min_connections: Some(10),
            url: std::env::var("DATABASE_URL").expect("DATABASE_URL"),
        },
    ));

    rocket::build()
        .configure(figment)
        .attach(DbSqlx::init())
        .attach(RemoveServerHeader)
        .attach(Template::fairing())
        .attach(Compression::fairing())
        .register("/", catchers![internal_error, not_found, default])
        .mount("/", routes![index])
}

#[catch(500)]
pub fn internal_error() -> &'static str {
    "internal server error"
}

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("I couldn't find '{}'. Try something else?", req.uri())
}

#[catch(default)]
pub fn default(status: Status, req: &Request) -> String {
    format!("{} ({})", status, req.uri())
}

async fn _stream_consume() {
    let rabbitmq = RabbitMqAdapter::new().await;
    let _queue_stream = rabbitmq.create_stream("stream", 1).await;
    let _handler = rabbitmq
        .consumer("stream", None, |delivery| async move {
            println!(
                "message {:?} with offset {}",
                delivery
                    .message()
                    .data()
                    .map(serde_json::from_slice::<Value>)
                    .unwrap(),
                delivery.offset()
            );
        })
        .await;
}
