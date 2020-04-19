use serde::{Deserialize, Serialize};

use super::super::schema::category;
use super::super::schema::recipe_category;
use super::model_recipe::*;

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
