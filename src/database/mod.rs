extern crate diesel_migrations;
extern crate dotenv;
mod models;
mod schema;

use diesel::prelude::*;
use diesel_migrations::*;
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
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn setup_test_db() -> SqliteConnection {
        let connection = SqliteConnection::establish(":memory:").unwrap();
        run_pending_migrations(&connection).expect("Fail to run migrations");
        connection
    }

    fn reset_db(connection: &SqliteConnection) {
        use self::schema::category::dsl::*;
        use self::schema::recipe::dsl::*;
        use self::schema::recipe_category::dsl::*;

        diesel::delete(recipe_category)
            .execute(connection)
            .expect("could not delete recipe_category associations");
        diesel::delete(category)
            .execute(connection)
            .expect("could not delete category");
        diesel::delete(recipe)
            .execute(connection)
            .expect("could not delete recipe");
    }

    #[test]
    fn test_category_insert() {
        setup_test_db();
        assert_eq!(3, 3);
    }
}
