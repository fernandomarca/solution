#![allow(dead_code)]

use crate::middler::ApiKey;
use crate::middler::ApiKeyError;
use crate::middler::MyResult;
use rocket::data::ToByteUnit;
use rocket::form::Form;
use rocket::form::Strict;
use rocket::fs::TempFile;
use rocket::http::CookieJar;
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::tokio;
use rocket::tokio::time::{sleep, Duration};
use rocket::Data;
use rocket_dyn_templates::context;
use rocket_dyn_templates::Template;
use serde::Deserialize;
use serde::Serialize;
extern crate rocket_db_pools;

use crate::db::schema::posts;
use crate::db::Db;
use crate::posts::Post;
use rocket::response::{status::Created, Debug};
use rocket_db_pools::diesel::prelude::*;
use rocket_db_pools::Connection;

#[get("/")]
fn index(cookies: &CookieJar<'_>) -> &'static str {
    cookies.add_private(("message", "hello!"));
    "Hello, world!"
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[get("/sensitive")]
fn sensitive(key: Result<ApiKey<'_>, ApiKeyError>, cookies: &CookieJar<'_>) -> MyResult {
    match key {
        Ok(k) => {
            let crumb = cookies
                .get_private("message")
                .map(|crumb| format!("Message: {}", crumb.value()));
            MyResult::Ok(format!("Your key is: {} - {:?}", k.0, crumb))
        }
        Err(e) => match e {
            ApiKeyError::Missing => MyResult::Redirect(Box::new(Redirect::to(uri!(index)))),
            _ => MyResult::Err(format!("Invalid key, {:?}", e)),
        },
    }
}

#[derive(Deserialize, Serialize)]
struct Task<'r> {
    description: &'r str,
    complete: bool,
}

#[post("/todo", data = "<task>")]
fn new(task: Json<Task<'_>>) -> Json<Task> {
    println!("Task: {} - {}", task.description, task.complete);
    task
}

#[post("/upload", format = "plain", data = "<file>")]
async fn upload(mut file: TempFile<'_>) -> std::io::Result<()> {
    file.persist_to("tmp/texto.txt").await
}

#[post("/debug", data = "<data>")]
async fn debug(data: Data<'_>) -> std::io::Result<()> {
    // Stream at most 512KiB all of the body data to stdout.
    data.open(512.kibibytes())
        .stream_to(tokio::io::stdout())
        .await?;

    Ok(())
}

#[derive(Deserialize, Serialize, FromForm, Debug)]
struct Todo<'r> {
    complete: bool,
    r#type: &'r str,
}

#[post("/new_todo", data = "<task>")]
fn new_todo(task: Form<Strict<Todo<'_>>>) -> Json<Todo> {
    println!("Task: {:?} ", task);
    let todo = Todo {
        complete: task.complete,
        r#type: task.r#type,
    };
    Json(todo)
}

#[derive(FromForm, Debug)]
struct UploadM<'r> {
    save: bool,
    file: TempFile<'r>,
}

#[post("/upload_form", data = "<upload>")]
async fn upload_form(mut upload: Form<UploadM<'_>>) {
    let _r = upload.file.persist_to("tmp/texto2.txt").await;
    println!("Upload: {:?}", upload.save);
}

#[derive(Responder)]
#[response(status = 418, content_type = "json")]
struct RawTeapotJson(&'static str);

#[get("/json")]
fn json() -> RawTeapotJson {
    RawTeapotJson("{ \"hi\": \"world\" }")
}

#[get("/template?<name>", data = "<post>")]
fn template(name: Option<&str>, post: Json<Post>) -> Template {
    let context = context! {
    name:name.or(Some("Guest")),
    title:post.title.to_owned(),
    items: vec![post.title.to_owned(), post.text.to_owned()]
    };
    Template::render("index", context)
}

#[get("/<__id>/<name>?<__age>")]
fn person(__id: Option<usize>, name: Option<&str>, __age: Option<u8>) -> Redirect {
    let url = uri!("/", template(name));
    println!("{}", url);
    Redirect::to(url)
}

#[get("/about")]
pub fn about() -> Template {
    Template::render(
        "about",
        context! {
            title: "About",
            parent: "layout",
        },
    )
}

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[post("/create", data = "<post>")]
async fn create(mut db: Connection<Db>, post: Json<Post>) -> Result<Created<Json<Post>>> {
    let post = db
        .transaction(|mut conn| {
            Box::pin(async move {
                diesel::insert_into(posts::table)
                    .values(&*post)
                    .execute(&mut conn)
                    .await?;

                Ok::<_, diesel::result::Error>(post)
            })
        })
        .await?;

    Ok(Created::new("/template").body(post))
}

#[get("/list")]
async fn list(mut db: Connection<Db>) -> Result<Json<Vec<i64>>> {
    let ids: Vec<i64> = posts::table.select(posts::id).load(&mut db).await?;
    Ok(Json(ids))
}
