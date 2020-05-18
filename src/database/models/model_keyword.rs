use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::super::schema::keyword;
use super::super::schema::recipe_keyword;
use super::model_recipe::*;

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize)]
#[belongs_to(Recipe)]
#[belongs_to(Keyword)]
#[table_name = "recipe_keyword"]
pub struct RecipeKeyword {
    pub id: i32,
    pub recipe_id: i32,
    pub keyword_id: i32,
}

#[derive(Insertable)]
#[table_name = "recipe_keyword"]
pub struct NewRecipeKeyword {
    pub recipe_id: i32,
    pub keyword_id: i32,
}

#[derive(Debug, Identifiable, Queryable, Associations, Serialize, Deserialize, JsonSchema)]
#[table_name = "keyword"]
pub struct Keyword {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Serialize)]
#[table_name = "keyword"]
pub struct NewKeyword<'a> {
    pub name: &'a str,
}
