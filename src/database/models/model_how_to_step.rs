use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::super::schema::how_to_step;
use super::super::schema::recipe_how_to_section_how_to_step;
use super::model_how_to_section::*;

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize)]
#[belongs_to(RecipeHowToSection)]
#[belongs_to(HowToStep)]
#[table_name = "recipe_how_to_section_how_to_step"]
pub struct RecipeHowToStep {
    pub id: i32,
    pub recipe_how_to_section_id: i32,
    pub how_to_step_id: i32,
}

#[derive(Insertable)]
#[table_name = "recipe_how_to_section_how_to_step"]
pub struct NewRecipeHowToStep {
    pub recipe_how_to_section_id: i32,
    pub how_to_step_id: i32,
}

#[derive(Debug, Identifiable, Queryable, Associations, Serialize, Deserialize, JsonSchema)]
#[table_name = "how_to_step"]
pub struct HowToStep {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Serialize)]
#[table_name = "how_to_step"]
pub struct NewHowToStep<'a> {
    pub name: &'a str,
}
