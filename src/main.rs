#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
mod database;

use rocket_contrib::json;
use database::get_recipes;

#[get("/")]
fn index() -> json::JsonValue {
    let haha = get_recipes();
    return json!(haha.get(0));
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}