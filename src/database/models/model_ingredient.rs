use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::super::schema::ingredient;
use super::super::schema::recipe_ingredient;
use super::model_recipe::*;

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize)]
#[belongs_to(Recipe)]
#[belongs_to(Ingredient)]
#[table_name = "recipe_ingredient"]
pub struct RecipeIngredient {
    pub id: i32,
    pub recipe_id: i32,
    pub ingredient_id: i32,
}

#[derive(Insertable)]
#[table_name = "recipe_ingredient"]
pub struct NewRecipeIngredient {
    pub recipe_id: i32,
    pub ingredient_id: i32,
}

#[derive(Debug, Identifiable, Queryable, Associations, Serialize, Deserialize, JsonSchema)]
#[table_name = "ingredient"]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Serialize)]
#[table_name = "ingredient"]
pub struct NewIngredient<'a> {
    pub name: &'a str,
}
