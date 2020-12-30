#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket_contrib::templates::Template;

#[get("/")]
fn index() -> &'static str {
"Hello, world!"
}

#[get("/<username>")]
fn read_user_name(username: String) -> Template {
    format!("Test {}", username.as_str())
}

#[post("/login", data="<username>, <password>")]
fn login_form(username: Form<username>, password: Form<password>) -> Template {
    Template::render("login", username, password)
}

fn main() {
    rocket::ignite().mount("/", routes![index, read_user_name]).launch();
}
