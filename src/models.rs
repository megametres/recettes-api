use super::schema::recipes;

#[derive(Queryable)]
pub struct Recipe {
    pub id: i32,
    pub title: String,
    pub body: String
}