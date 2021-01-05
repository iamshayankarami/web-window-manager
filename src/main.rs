#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket_contrib::templates::Template;
use serde::Serialize;

#[get("/")]
fn index() -> &'static str {
    "Shayan is the best programmer"
}

#[get("/<username>")]
fn read_user_name(username: String) -> Template {
    #[derive(Serialize)]
    struct Context {
        username: String
    }

    let con = Context {username: String::from(username)};
    Template::render("index", con)
}

fn main() {
    rocket::ignite().mount("/", routes![index, read_user_name]).attach(Template::fairing()).launch();
}
