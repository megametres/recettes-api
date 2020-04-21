use super::model_how_to_step::*;

pub struct RecipeHowToSectionFull {
    pub id: i32,
    pub name: String,
    pub how_to_steps: Vec<HowToStep>,
}
