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

use rocket::fairing::{Fairing, Info, Kind};
use rocket::fs::{relative, FileServer};
use rocket::http::Header;
use rocket::{Request, Response};
use rocket_dyn_templates::{context, Template};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

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
                api::api::get_entries,
                api::api::get_all,
                api::api::get_recent,
                api::api::get_top_tiers,
                api::api::get_popular,
                api::api::get_top_rated,
                api::api::search,
                api::api::search_options,
                api::api::entry,
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
        .attach(CORS)
        .manage(db)
}
