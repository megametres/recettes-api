use serde::{Deserialize, Serialize};

use super::schema::category;
use super::schema::recipe;
use super::schema::recipe_category;

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize)]
#[table_name = "recipe"]
pub struct Recipe {
    pub id: i32,
    pub name: String,
    pub author: String,
    pub image: String,
    pub prep_time: String,
    pub cook_time: String,
    pub total_time: String,
    pub recipe_yield: String,
    pub description: String,
    pub json_ld: String,
}

#[derive(Insertable)]
#[table_name = "recipe"]
pub struct NewRecipe<'a> {
    pub name: &'a str,
    pub author: &'a str,
    pub image: &'a str,
    pub prep_time: &'a str,
    pub cook_time: &'a str,
    pub total_time: &'a str,
    pub recipe_yield: &'a str,
    pub description: &'a str,
    pub json_ld: &'a str,
}

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize)]
#[belongs_to(Recipe)]
#[belongs_to(Category)]
#[table_name = "recipe_category"]
pub struct RecipeCategory {
    pub id: i32,
    pub recipe_id: i32,
    pub category_id: i32,
}

#[derive(Insertable)]
#[table_name = "recipe_category"]
pub struct NewRecipeCategory {
    pub recipe_id: i32,
    pub category_id: i32,
}

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize)]
#[table_name = "category"]
pub struct Category {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Serialize)]
#[table_name = "category"]
pub struct NewCategory<'a> {
    pub name: &'a str,
}
