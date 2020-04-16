extern crate dotenv;

mod schema;
mod models;

use rocket_contrib::json;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use models::*;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn get_recipes() -> Vec<Recipe> {

    use self::schema::recipes::dsl::*;

    let connection = establish_connection();

    return recipes
        .load::<Recipe>(&connection)
        .expect("Error loading recipes");
    
}


pub fn get_recipes2 () -> json::JsonValue {
    json!({ "status": "ok" })
}