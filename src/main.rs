#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

mod admin;
mod api;
mod db;
mod guards;
mod models;

use db::DB;

use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::{context, Template};

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
}

#[launch]
async fn rocket() -> _ {
    let db = DB::connect(
        &std::env::var("MONGO_URL").expect("MONGO_URL environment variable is required"),
    )
    .await
    .unwrap();

    rocket::build()
        .mount(
            "/",
            routes![
                index,
                api::api::get_tags,
                api::api::get_authors,
                api::api::get_entries
            ],
        )
        .mount(
            "/admin",
            routes![
                admin::index,
                admin::login,
                admin::login_post,
                admin::add_tag,
                admin::add_tag_post,
                admin::delete_tag_post,
                admin::add_author,
                admin::add_author_post,
                admin::add_entry,
                admin::add_entry_post,
            ],
        )
        .mount("/static", FileServer::from(relative!("static")))
        .attach(Template::fairing())
        .manage(db)
}
