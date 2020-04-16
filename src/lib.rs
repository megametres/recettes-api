#[macro_use] extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

mod schema;
mod models;

use super::schema::recipes;
use models::*;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn get_recipes() -> Vec<Recipe> {

    use recipes_api::schema::recipes::dsl::*;
    let connection = establish_connection();

    let result = models::recipes::recipes.filter(id.eq(1))
        .limit(5)
        .load::<Recipe>(&connection)
        .expect("Error loading recipes");
}
