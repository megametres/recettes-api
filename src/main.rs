#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_okapi;
#[macro_use]
extern crate diesel;
mod database;

use database::get_recipes;
use rocket_contrib::json;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

#[openapi]
#[get("/")]
fn index() -> json::JsonValue {
    let haha = get_recipes();
    return json!(haha.get(0));
}

fn get_docs() -> SwaggerUIConfig {
    SwaggerUIConfig {
        url: Some("/openapi.json".to_owned()),
        urls: None,
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes_with_openapi![index])
        .mount("/swagger", make_swagger_ui(&get_docs()))
        .launch();
}
