use serde::{Deserialize, Serialize};

use super::super::schema::recipe;

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize)]
#[table_name = "recipe"]
pub struct Recipe {
    pub id: i32,
    pub name: String,
    pub author: Option<String>,
    pub image: Option<String>,
    pub prep_time: Option<String>,
    pub cook_time: Option<String>,
    pub total_time: Option<String>,
    pub recipe_yield: Option<String>,
    pub description: String,
    pub json_ld: String,
}

#[derive(Insertable)]
#[table_name = "recipe"]
pub struct NewRecipe<'a> {
    pub name: &'a str,
    pub author: Option<&'a str>,
    pub image: Option<&'a str>,
    pub prep_time: Option<&'a str>,
    pub cook_time: Option<&'a str>,
    pub total_time: Option<&'a str>,
    pub recipe_yield: Option<&'a str>,
    pub description: &'a str,
    pub json_ld: &'a str,
}
