#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_okapi;
#[macro_use]
extern crate diesel;
mod database;
mod error;
mod html_parser;

use database::*;
use html_parser::*;
use rocket_contrib::json;
use rocket_contrib::json::Json;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, JsonSchema)]
struct InputUrl {
    value: String,
}

#[openapi]
#[post("/parse_recipe", data = "<input_url>")]
fn parse_recipe(input_url: Json<InputUrl>) {
    let input: InputUrl = input_url.into_inner();
    match recipe_parser(input.value.as_str()) {
        Ok(recipe) => database::save_recipe(recipe),
        Err(e) => println!("{}", e),
    }
}

fn get_docs() -> SwaggerUIConfig {
    SwaggerUIConfig {
        url: Some("/openapi.json".to_owned()),
        urls: None,
    }
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes_with_openapi![index, recipe, recipe_list, parse_recipe],
        )
        .mount("/swagger", make_swagger_ui(&get_docs()))
        .launch();
}
