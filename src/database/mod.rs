extern crate diesel_migrations;
extern crate dotenv;
pub mod models;
pub mod recipe_helper;
mod schema;

use models::model_recipe::*;
use recipe_helper::*;

pub fn get_recipes() -> Vec<RecipeSimple> {
    let connection = establish_connection();
    get_recipe_list(&connection)
}

pub fn get_recipe(recipe_id: i32) -> RecipeFull {
    let connection = establish_connection();
    load_recipe(&connection, recipe_id)
}

pub fn save_recipe(recipe: RecipeFull) {
    let connection = establish_connection();
    recipe_helper::save_recipe(&connection, &recipe);
}

pub fn edit_recipe(recipe_id: i32, recipe: RecipeFull) -> Result<(), Box<dyn std::error::Error>> {
    let connection = establish_connection();
    recipe_helper::delete_recipe(&connection, recipe_id)?;
    recipe_helper::save_recipe(&connection, &recipe);
    Ok(())
}

pub fn delete_recipe(recipe_id: i32) -> Result<(), Box<dyn std::error::Error>> {
    let connection = establish_connection();
    recipe_helper::delete_recipe(&connection, recipe_id)
}
