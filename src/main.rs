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

use database::models::model_input_url::*;
use database::models::model_recipe::*;
use database::*;
use html_parser::*;
use rocket::http::Method;
// use rocket::http::Status;
use rocket_contrib::json;
use rocket_contrib::json::Json;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions, Error};
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use std::env;

fn make_cors() -> Cors {
    let frontend_url;
    match env::var("FRONTEND_URL") {
        Ok(url) => frontend_url = [url],
        Err(_) => frontend_url = ["http://127.0.0.1:4200".to_string()],
    };

    let allowed_origins = AllowedOrigins::some_exact(&frontend_url);

    CorsOptions {
        allowed_origins,
        allowed_methods: vec![
            Method::Get,
            Method::Post,
            Method::Put,
            Method::Delete,
            Method::Options,
        ]
        .into_iter()
        .map(From::from)
        .collect(),
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin",
            "Content-Type",
            "Content-Length",
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS")
}

#[openapi]
#[get("/recipes?<category_id>")]
fn list_recipes(category_id: Option<i32>) -> json::JsonValue {
    let return_element = get_recipes(category_id);
    return json!(return_element);
}

#[openapi]
#[get("/categories")]
fn list_categories() -> json::JsonValue {
    let return_element = get_categories();
    return json!(return_element);
}

#[openapi]
#[get("/recipe/<id>")]
fn get_recipe(id: i32) -> json::JsonValue {
    let return_element = database::get_recipe(id);
    return json!(return_element);
}

#[openapi]
#[post("/add_recipe", data = "<recipe>")]
fn add_recipe(recipe: Json<RecipeFull>) {
    database::save_recipe(recipe.into_inner());
}

#[openapi]
#[post("/parse_recipe", data = "<input_url>")]
fn parse_recipe(input_url: Json<InputUrl>) -> json::JsonValue {
    let input: InputUrl = input_url.into_inner();
    match recipe_parser(input.url.as_str()) {
        Ok(recipe) => return json!(database::save_recipe(recipe)),
        Err(_) => return json!(-1),
    }
}

#[openapi]
#[put("/save_recipe/<id>", format = "application/json", data = "<recipe>")]
fn edit_recipe(id: i32, recipe: Json<RecipeFull>) -> json::JsonValue {
    match database::edit_recipe(id, recipe.into_inner()) {
        Ok(recipe_id) => return json!(recipe_id),
        Err(_) => return json!(-1),
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
        url: "/openapi.json".to_owned(),
        ..Default::default()
    }
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes_with_openapi![
                add_recipe,
                edit_recipe,
                get_recipe,
                list_recipes,
                parse_recipe,
                delete_recipe,
                list_categories
            ],
        )
        .mount("/", make_swagger_ui(&get_docs()))
        .attach(make_cors())
        .launch();
}
