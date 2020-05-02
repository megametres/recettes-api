use super::model_category::*;
use super::model_how_to_section_full::*;
use super::model_ingredient::*;
use super::model_keyword::*;

pub struct RecipeFull {
    pub id: i32,
    pub name: String,
    pub author: Option<String>,
    pub image: Option<String>,
    pub prep_time: Option<String>,
    pub cook_time: Option<String>,
    pub total_time: Option<String>,
    pub recipe_yield: Option<String>,
    pub description: String,
    pub categories: Option<Vec<Category>>,
    pub keywords: Option<Vec<Keyword>>,
    pub ingredients: Option<Vec<Ingredient>>,
    pub how_to_section_full: Option<Vec<RecipeHowToSectionFull>>,
}
