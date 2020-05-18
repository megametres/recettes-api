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

use database::models::model_recipe::*;
use database::*;
use html_parser::*;
use rocket::http::Status;
use rocket_contrib::json;
use rocket_contrib::json::Json;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, JsonSchema)]
struct InputUrl {
    url: String,
}

#[openapi]
#[get("/recipes")]
fn list_recipes() -> json::JsonValue {
    let return_element = get_recipes();
    return json!(return_element);
}

#[openapi]
#[get("/recipe/<id>")]
fn get_recipe(id: i32) -> json::JsonValue {
    let return_element = get_recipe(id);
    return json!(return_element);
}

#[openapi]
#[post("/add_recipe", data = "<recipe>")]
fn add_recipe(recipe: Json<RecipeFull>) {
    database::save_recipe(recipe.into_inner());
}

#[openapi]
#[post("/parse_recipe", data = "<input_url>")]
fn parse_recipe(input_url: Json<InputUrl>) {
    let input: InputUrl = input_url.into_inner();
    match recipe_parser(input.url.as_str()) {
        Ok(recipe) => database::save_recipe(recipe),
        Err(e) => println!("{}", e),
    }
}

#[openapi]
#[delete("/delete_recipe/<recipe_id>")]
fn delete_recipe(recipe_id: i32) {
    match database::delete_recipe(recipe_id) {
        Ok(()) => (),
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
            routes_with_openapi![
                add_recipe,
                get_recipe,
                list_recipes,
                parse_recipe,
                delete_recipe
            ],
        )
        .mount("/", make_swagger_ui(&get_docs()))
        .launch();
}
