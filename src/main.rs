#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::json;

#[get("/")]
fn index() -> json::JsonValue {
    json!({ "status": "ok" })
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}