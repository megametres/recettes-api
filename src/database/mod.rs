extern crate diesel_migrations;
extern crate dotenv;
mod models;
mod recipe_helper;
mod schema;

use models::model_recipe::*;
use recipe_helper::*;

pub fn get_recipes() -> Vec<RecipeSimple> {
    let connection = establish_connection();
    get_recipe_list(&connection)
}

pub fn get_recipe(index: i32) -> RecipeFull {
    let connection = establish_connection();
    load_recipe(&connection, index)
}
