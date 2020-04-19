extern crate diesel_migrations;
extern crate dotenv;
mod models;
mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use models::*;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_recipes() -> Vec<Recipe> {
    use self::schema::recipe::dsl::*;

    let connection = establish_connection();

    recipe
        .load::<Recipe>(&connection)
        .expect("Error loading recipes")
}

#[cfg(test)]
mod tests {
    use self::schema::category::dsl::*;
    use self::schema::recipe::dsl::*;
    use self::schema::recipe_category::dsl::*;
    use super::*;
    use diesel::dsl::count;
    use diesel_migrations::*;

    fn setup_test_db() -> SqliteConnection {
        let connection = SqliteConnection::establish(":memory:").unwrap();
        run_pending_migrations(&connection).expect("Fail to run migrations");
        connection
    }

    fn dummy_category_a<'a>() -> NewCategory<'a> {
        NewCategory { name: "Category A" }
    }

    fn dummy_category_b<'a>() -> NewCategory<'a> {
        NewCategory { name: "Category B" }
    }

    fn dummy_recipe_a<'a>() -> NewRecipe<'a> {
        NewRecipe {
            name: "Recipe A",
            author: "Recipe A author",
            image: "Recipe A image",
            prep_time: "Recipe A prep_time",
            cook_time: "Recipe A cook_time",
            total_time: "Recipe A total_time",
            recipe_yield: "Recipe A recipe_yield",
            description: "Recipe A description",
            json_ld: "Recipe A json_ld",
        }
    }

    fn dummy_recipe_category_a(connection: &SqliteConnection) -> NewRecipeCategory {
        use schema::*;
        let test_category_a = dummy_category_a();
        let test_recipe_a = dummy_recipe_a();

        // Inserting data in the database
        diesel::insert_into(category::table)
            .values(&test_category_a)
            .execute(connection)
            .unwrap_or_else(|_| panic!("{}{}", "Error saving new category ", test_category_a.name));

        let inserted_category = category
            .filter(category::name.eq(test_category_a.name))
            .load::<Category>(connection)
            .unwrap_or_else(|_| panic!("{}{}", "Error loading category ", test_category_a.name));

        diesel::insert_into(recipe::table)
            .values(&test_recipe_a)
            .execute(connection)
            .unwrap_or_else(|_| panic!("{}{}", "Error saving new recipe ", test_recipe_a.name));

        let inserted_recipe = recipe
            .filter(recipe::name.eq(test_recipe_a.name))
            .load::<Recipe>(connection)
            .unwrap_or_else(|_| panic!("{}{}", "Error loading recipe ", test_recipe_a.name));

        NewRecipeCategory {
            recipe_id: inserted_recipe.get(0).unwrap().id,
            category_id: inserted_category.get(0).unwrap().id,
        }
    }

    #[test]
    fn test_category_insert() {
        use schema::category;

        let connection = setup_test_db();

        let test_category_a = dummy_category_a();

        // Inserting data in the database
        diesel::insert_into(category::table)
            .values(&test_category_a)
            .execute(&connection)
            .unwrap_or_else(|_| panic!("{}{}", "Error saving new category ", test_category_a.name));

        // Retrieving data from the database
        let result = category
            .filter(category::name.eq(test_category_a.name))
            .load::<Category>(&connection)
            .unwrap_or_else(|_| panic!("{}{}", "Error loading category ", test_category_a.name));

        // Compare created data with the database value
        assert_eq!(test_category_a.name, result.get(0).unwrap().name);
    }

    #[test]
    fn test_category_multiple_insert() {
        use schema::category;

        let connection = setup_test_db();

        let test_category_a = dummy_category_a();
        let test_category_b = dummy_category_b();

        // Inserting data in the database
        diesel::insert_into(category::table)
            .values(&test_category_a)
            .execute(&connection)
            .unwrap_or_else(|_| panic!("{}{}", "Error saving category ", test_category_a.name));

        diesel::insert_into(category::table)
            .values(&test_category_b)
            .execute(&connection)
            .unwrap_or_else(|_| panic!("{}{}", "Error saving category ", test_category_b.name));

        // Retrieving data from the database
        let result_a = category
            .filter(category::name.eq(test_category_a.name))
            .load::<Category>(&connection)
            .unwrap_or_else(|_| panic!("{}{}", "Error loading category ", test_category_a.name));

        let result_b = category
            .filter(category::name.eq(test_category_b.name))
            .load::<Category>(&connection)
            .unwrap_or_else(|_| panic!("{}{}", "Error loading category ", test_category_b.name));

        // Compare created data with the database value
        assert_eq!(test_category_a.name, result_a.get(0).unwrap().name);
        assert_eq!(test_category_b.name, result_b.get(0).unwrap().name);

        // Validate the number of row created
        assert_eq!(
            Ok(2),
            category.select(count(category::name)).first(&connection)
        );
    }

    #[test]
    fn test_recipe_insert() {
        use schema::recipe;

        let connection = setup_test_db();

        let test_recipe_a = dummy_recipe_a();

        diesel::insert_into(recipe::table)
            .values(&test_recipe_a)
            .execute(&connection)
            .unwrap_or_else(|_| panic!("{}{}", "Error saving new recipe ", test_recipe_a.name));

        let result = recipe
            .filter(recipe::name.eq(test_recipe_a.name))
            .load::<Recipe>(&connection)
            .unwrap_or_else(|_| panic!("{}{}", "Error loading recipe ", test_recipe_a.name));

        assert_eq!(test_recipe_a.name, result.get(0).unwrap().name);
        assert_eq!(test_recipe_a.author, result.get(0).unwrap().author);
        assert_eq!(test_recipe_a.image, result.get(0).unwrap().image);
        assert_eq!(test_recipe_a.prep_time, result.get(0).unwrap().prep_time);
        assert_eq!(test_recipe_a.cook_time, result.get(0).unwrap().cook_time);
        assert_eq!(test_recipe_a.total_time, result.get(0).unwrap().total_time);
        assert_eq!(
            test_recipe_a.recipe_yield,
            result.get(0).unwrap().recipe_yield
        );
        assert_eq!(
            test_recipe_a.description,
            result.get(0).unwrap().description
        );
        assert_eq!(test_recipe_a.json_ld, result.get(0).unwrap().json_ld);
    }

    #[test]
    fn test_recipe_category_insert() {
        use schema::*;

        let connection = setup_test_db();

        let test_recipe_category_a = dummy_recipe_category_a(&connection);

        // Inserting data in the database
        diesel::insert_into(recipe_category::table)
            .values(&test_recipe_category_a)
            .execute(&connection)
            .unwrap_or_else(|_| {
                panic!(
                    "{} ({},{})",
                    "Error saving new recipe_category ",
                    test_recipe_category_a.recipe_id,
                    test_recipe_category_a.category_id,
                )
            });

        // Retrieving data from the database
        let result = recipe_category
            .filter(recipe_category::recipe_id.eq(test_recipe_category_a.recipe_id))
            .filter(recipe_category::recipe_id.eq(test_recipe_category_a.recipe_id))
            .load::<RecipeCategory>(&connection)
            .unwrap_or_else(|_| {
                panic!(
                    "{} ({},{})",
                    "Error loading recipe_category ",
                    test_recipe_category_a.recipe_id,
                    test_recipe_category_a.category_id,
                )
            });

        // Compare created data with the database value
        assert_eq!(
            test_recipe_category_a.recipe_id,
            result.get(0).unwrap().recipe_id
        );
        assert_eq!(
            test_recipe_category_a.category_id,
            result.get(0).unwrap().category_id
        );
    }
}
