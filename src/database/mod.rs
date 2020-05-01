extern crate diesel_migrations;
extern crate dotenv;
mod models;
mod recipe_helper;
mod schema;

use diesel::prelude::*;
use models::model_recipe::*;
use recipe_helper::*;

pub fn get_recipes() -> Vec<Recipe> {
    use self::schema::recipe::dsl::*;

    let connection = establish_connection();

    load_recipe(&connection, 1);

    recipe
        .load::<Recipe>(&connection)
        .expect("Error loading recipes")
}
