#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_okapi;
#[macro_use]
extern crate diesel;
mod database;

use database::*;
use rocket_contrib::json;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

#[openapi]
#[get("/")]
fn index() -> json::JsonValue {
    let return_element = get_recipes();
    return json!(return_element.get(0));
}

#[openapi]
#[get("/recipes")]
fn recipe_list() -> json::JsonValue {
    let return_element = get_recipes();
    return json!(return_element.get(0));
}

#[openapi]
#[get("/recipe/<id>")]
fn recipe(id: i32) -> json::JsonValue {
    let return_element = get_recipe(id);
    return json!(return_element);
}

fn get_docs() -> SwaggerUIConfig {
    SwaggerUIConfig {
        url: Some("/openapi.json".to_owned()),
        urls: None,
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes_with_openapi![index, recipe, recipe_list])
        .mount("/swagger", make_swagger_ui(&get_docs()))
        .launch();
}
