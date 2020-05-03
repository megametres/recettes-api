use serde::{Deserialize, Serialize};

use super::super::schema::how_to_section;
use super::super::schema::recipe_how_to_section;
use super::model_how_to_step::*;
use super::model_recipe::*;

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize)]
#[belongs_to(Recipe)]
#[belongs_to(HowToSection)]
#[table_name = "recipe_how_to_section"]
pub struct RecipeHowToSection {
    pub id: i32,
    pub recipe_id: i32,
    pub how_to_section_id: i32,
}

#[derive(Insertable)]
#[table_name = "recipe_how_to_section"]
pub struct NewRecipeHowToSection {
    pub recipe_id: i32,
    pub how_to_section_id: i32,
}

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize)]
#[table_name = "how_to_section"]
pub struct HowToSection {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Serialize)]
#[table_name = "how_to_section"]
pub struct NewHowToSection<'a> {
    pub name: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct RecipeHowToSectionFull {
    pub id: i32,
    pub name: String,
    pub how_to_steps: Vec<HowToStep>,
}
