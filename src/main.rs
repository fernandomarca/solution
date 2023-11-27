use solution::create_app::start_app;

#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> _ {
    start_app()
}
