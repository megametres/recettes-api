use super::model_category::*;
use super::model_how_to_section_full::*;
use super::model_ingredient::*;
use super::model_keyword::*;

pub struct RecipeFull {
    pub id: i32,
    pub name: String,
    pub author: String,
    pub image: String,
    pub prep_time: String,
    pub cook_time: String,
    pub total_time: String,
    pub recipe_yield: String,
    pub description: String,
    pub categories: Vec<Category>,
    pub keywords: Vec<Keyword>,
    pub ingredients: Vec<Ingredient>,
    pub how_to_section_full: Vec<RecipeHowToSectionFull>,
}
