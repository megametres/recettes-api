use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::super::schema::recipe;
use super::model_category::*;
use super::model_how_to_section::*;
use super::model_ingredient::*;
use super::model_keyword::*;

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize)]
#[belongs_to(Category)]
#[table_name = "recipe"]
pub struct Recipe {
    pub id: i32,
    pub name: String,
    pub category_id: i32,
    pub author: Option<String>,
    pub image: Option<String>,
    pub prep_time: Option<String>,
    pub cook_time: Option<String>,
    pub total_time: Option<String>,
    pub recipe_yield: Option<String>,
    pub description: String,
    pub json_ld: Option<String>,
    pub source: Option<String>,
}

#[derive(Insertable)]
#[table_name = "recipe"]
pub struct NewRecipe<'a> {
    pub name: &'a str,
    pub category_id: &'a i32,
    pub author: Option<&'a str>,
    pub image: Option<&'a str>,
    pub prep_time: Option<&'a str>,
    pub cook_time: Option<&'a str>,
    pub total_time: Option<&'a str>,
    pub recipe_yield: Option<&'a str>,
    pub description: &'a str,
    pub json_ld: Option<&'a str>,
    pub source: Option<&'a str>,
}

#[derive(Serialize, Queryable, Deserialize)]
pub struct RecipeSimple {
    pub id: i32,
    pub name: String,
    pub category: String,
}

#[derive(Default, Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct RecipeExtended {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub category: Option<String>,
    pub author: Option<String>,
    pub image: Option<String>,
    pub prep_time: Option<String>,
    pub cook_time: Option<String>,
    pub total_time: Option<String>,
    pub recipe_yield: Option<String>,
    pub description: Option<String>,
    pub keywords: Option<Vec<Keyword>>,
    pub ingredients: Option<Vec<Ingredient>>,
    pub how_to_section_full: Option<Vec<RecipeHowToSectionFull>>,
    pub json_ld: Option<String>,
    pub source: Option<String>,
}
