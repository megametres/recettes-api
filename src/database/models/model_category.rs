use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::super::schema::category;

#[derive(Debug, Identifiable, Queryable, Associations, Serialize, Deserialize, JsonSchema)]
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
