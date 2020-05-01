extern crate diesel_migrations;
extern crate dotenv;

use super::models::model_category::*;
use super::models::model_how_to_section::*;
use super::models::model_how_to_section_full::*;
use super::models::model_how_to_step::*;
use super::models::model_ingredient::*;
use super::models::model_keyword::*;
use super::models::model_recipe::*;
use super::models::model_recipe_full::*;

use super::schema::recipe::dsl::*;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

macro_rules! load_recipe_table {
    ($connection:expr, $belongs_to:expr, $return_model:ty, $load_model:ty, $join_table:expr, $select_table:expr) => {
        <$return_model>::belonging_to($belongs_to)
            .inner_join($join_table)
            .select($select_table)
            .load::<$load_model>($connection)
            .unwrap();
    };
}

pub fn load_recipe(connection: &PgConnection, recipe_id: i32) -> RecipeFull {
    use crate::database::schema::*;

    let queried_recipe = recipe
        .find(recipe_id)
        .get_result::<Recipe>(connection)
        .unwrap();

    let queried_category = load_recipe_table!(
        connection,
        &queried_recipe,
        RecipeCategory,
        Category,
        category::table,
        category::all_columns
    );

    let queried_ingredient = load_recipe_table!(
        connection,
        &queried_recipe,
        RecipeIngredient,
        Ingredient,
        ingredient::table,
        ingredient::all_columns
    );

    let queried_keyword = load_recipe_table!(
        connection,
        &queried_recipe,
        RecipeKeyword,
        Keyword,
        keyword::table,
        keyword::all_columns
    );

    let queried_how_to_section = load_recipe_table!(
        connection,
        &queried_recipe,
        RecipeHowToSection,
        HowToSection,
        how_to_section::table,
        how_to_section::all_columns
    );

    let queried_recipe_how_to_section = load_recipe_table!(
        connection,
        &queried_recipe,
        RecipeHowToSection,
        RecipeHowToSection,
        how_to_section::table,
        recipe_how_to_section::all_columns
    );

    let mut queried_how_to_section_full: Vec<RecipeHowToSectionFull> = Vec::new();

    for i in 0..queried_recipe_how_to_section.len() {
        let queried_how_to_step = load_recipe_table!(
            connection,
            queried_recipe_how_to_section.get(i).unwrap(),
            RecipeHowToStep,
            HowToStep,
            how_to_step::table,
            how_to_step::all_columns
        );
        queried_how_to_section_full.push(RecipeHowToSectionFull {
            id: queried_recipe_how_to_section.get(0).unwrap().id,
            name: queried_how_to_section.get(i).unwrap().name.to_owned(),
            how_to_steps: queried_how_to_step,
        });
    }

    RecipeFull {
        id: queried_recipe.id,
        name: queried_recipe.name,
        author: queried_recipe.author,
        image: queried_recipe.image,
        prep_time: queried_recipe.prep_time,
        cook_time: queried_recipe.cook_time,
        total_time: queried_recipe.total_time,
        recipe_yield: queried_recipe.recipe_yield,
        description: queried_recipe.description,
        categories: queried_category,
        keywords: queried_keyword,
        ingredients: queried_ingredient,
        how_to_section_full: queried_how_to_section_full,
    }
}

macro_rules! upsert_recipe_elements {
    ($connection:expr, $table_name:ident, $model:ty, $elements:expr) => {{
        let mut return_elements = Vec::new();
        for x in $elements {
            let inserted_element: Vec<$model> = diesel::insert_into($table_name::table)
                .values($table_name::name.eq(x.name.to_owned()))
                .on_conflict($table_name::name)
                .do_update()
                .set($table_name::name.eq(excluded($table_name::name)))
                .returning(($table_name::id, $table_name::name))
                .get_results::<$model>($connection)
                .unwrap();
            return_elements.push(inserted_element[0].id);
        }
        return_elements
    }};
}

// macro_rules! link_recipe_elements {
//     ($connection:expr, $table_name:ident, $model:ty, $elements:expr) => {{
//         let mut return_elements = Vec::new();
//         for x in $elements {
//             let inserted_element: Vec<$model> = diesel::insert_into($table_name::table)
//                 .values($table_name::name.eq(x.name.to_owned()))
//                 .on_conflict($table_name::name)
//                 .do_update()
//                 .set($table_name::name.eq(excluded($table_name::name)))
//                 .returning(($table_name::id, $table_name::name))
//                 .get_results::<$model>($connection)
//                 .unwrap();
//             return_elements.push(inserted_element[0].id);
//         }
//         return_elements
//     }};
// }

pub fn save_recipe(connection: &PgConnection, recipe_to_save: RecipeFull) -> bool {
    use super::schema::category;
    use super::schema::how_to_section;
    use super::schema::ingredient;
    use super::schema::keyword;
    use diesel::pg::upsert::excluded;

    let inserted_categories =
        upsert_recipe_elements!(connection, category, Category, &recipe_to_save.categories);
    let inserted_ingredients = upsert_recipe_elements!(
        connection,
        ingredient,
        Ingredient,
        &recipe_to_save.ingredients
    );
    let inserted_keywords =
        upsert_recipe_elements!(connection, keyword, Keyword, &recipe_to_save.keywords);
    let inserted_how_to_section = upsert_recipe_elements!(
        connection,
        how_to_section,
        HowToSection,
        &recipe_to_save.how_to_section_full
    );

    // connection.build_transaction().read_write().run(|| {
    //     let inserted_categories: Vec<i32> = Vec::new();
    //     diesel::insert_into(category::table)
    //         .values(&test_category_a)
    //         .get_results(&connection)
    // });

    // connection
    //     .transaction::<_, Error, _>(|| {
    //         let ha = diesel::insert_into(category::table)
    //             .values(&test_category_a)
    //             .get_results(&connection)
    //             .unwrap_or_else(|_| {
    //                 panic!("{}{}", "Error saving new category ", test_category_a.name)
    //             });

    //         Ok(())
    //     })
    //     .unwrap_or_else(|_| panic!("{}", "Error saving new recipe"));

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::schema::category::dsl::*;
    use crate::database::schema::how_to_section::dsl::*;
    use crate::database::schema::how_to_step::dsl::*;
    use crate::database::schema::ingredient::dsl::*;
    use crate::database::schema::keyword::dsl::*;
    use crate::database::schema::recipe_category::dsl::*;
    use crate::database::schema::recipe_how_to_section::dsl::*;
    use crate::database::schema::recipe_how_to_section_how_to_step::dsl::*;
    use crate::database::schema::recipe_ingredient::dsl::*;
    use crate::database::schema::recipe_keyword::dsl::*;

    use diesel::dsl::count;
    use diesel_migrations::*;

    fn setup_test_db() -> PgConnection {
        dotenv().ok();

        let database_test_url =
            env::var("DATABASE_TEST_URL").expect("DATABASE_TEST_URL must be set");

        let connection = PgConnection::establish(&database_test_url).unwrap();

        run_pending_migrations(&connection).expect("Fail to run migrations");
        reset_db(&connection);
        connection
    }

    // fn empty_recipe_table<T: Identifiable>(connection: &PgConnection, table_name: T) {
    //     diesel::delete(table_name)
    //         .execute(connection)
    //         .expect(format!("could not delete {:?} table", table_name));
    // }

    macro_rules! empty_recipe_table {
        ($connection:expr, $table:expr) => {
            diesel::delete($table)
                .execute($connection)
                .expect("could not delete table");
        };
    }

    fn reset_db(connection: &PgConnection) {
        empty_recipe_table!(connection, recipe_category);
        empty_recipe_table!(connection, category);
        empty_recipe_table!(connection, recipe_keyword);
        empty_recipe_table!(connection, keyword);
        empty_recipe_table!(connection, recipe_ingredient);
        empty_recipe_table!(connection, ingredient);
        empty_recipe_table!(connection, recipe_how_to_section_how_to_step);
        empty_recipe_table!(connection, recipe_how_to_section);
        empty_recipe_table!(connection, how_to_section);
        empty_recipe_table!(connection, how_to_step);
        empty_recipe_table!(connection, recipe);
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
            author: "Recipe A authoString::from(",
            image: "Recipe A image",
            prep_time: "Recipe A prep_time",
            cook_time: "Recipe A cook_time",
            total_time: "Recipe A total_time",
            recipe_yield: "Recipe A recipe_yield",
            description: "Recipe A description",
            json_ld: "Recipe A json_ld",
        }
    }

    fn dummy_recipe_category_a(connection: &PgConnection) -> NewRecipeCategory {
        use crate::database::schema::*;
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
        use crate::database::schema::category;

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
        use crate::database::schema::category;

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
        use crate::database::schema::recipe;

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
        use crate::database::schema::*;

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
    #[test]
    fn test_save_recipe() {
        let connection = setup_test_db();

        let test_category = vec![Category {
            id: 1,
            name: String::from("Desserts"),
        }];

        let test_ingredient = vec![
            Ingredient {
                id: 1,
                name: String::from("375 ml (1 1/2 tasse) de farine tout usage non blanchie"),
            },
            Ingredient {
                id: 2,
                name: String::from("250 ml (1 tasse) de poudre d'amandes"),
            },
            Ingredient {
                id: 3,
                name: String::from("5 ml (1 c. à thé) de poudre à pâte"),
            },
            Ingredient {
                id: 4,
                name: String::from("1 ml (1/4 c. à thé) de sel"),
            },
            Ingredient {
                id: 5,
                name: String::from("180 ml (3/4 tasse) de beurre non salé, ramolli"),
            },
            Ingredient {
                id: 6,
                name: String::from("250 ml (1 tasse) de sucre en poudre"),
            },
            Ingredient {
                id: 7,
                name: String::from("15 ml (1 c. à soupe) d'eau froide"),
            },
            Ingredient {
                id: 8,
                name: String::from("5 ml (1 c. à thé) d'extrait de vanille"),
            },
            Ingredient {
                id: 9,
                name: String::from("1 ml (1/4 c. à thé) d'extrait d'amande (facultatif)"),
            },
        ];

        let test_keyword = vec![
            Keyword {
                id: 1,
                name: String::from("recettes de Noël"),
            },
            Keyword {
                id: 2,
                name: String::from("desserts de Noël"),
            },
            Keyword {
                id: 3,
                name: String::from("biscuits de Noël"),
            },
            Keyword {
                id: 4,
                name: String::from("biscuits sablés au beurre"),
            },
            Keyword {
                id: 5,
                name: String::from("recettes de biscuits sablés au beurre"),
            },
            Keyword {
                id: 6,
                name: String::from("biscuits"),
            },
            Keyword {
                id: 7,
                name: String::from("recettes de biscuits"),
            },
        ];

        let recipe_how_to_step = vec![
            HowToStep{
                id: 1,
                name: String::from("Dans un bol, mélanger la farine, la poudre d'amandes, la poudre à pâte et le sel. Réserver."),
            },
            HowToStep{
                id: 2,
                name: String::from("Dans un autre bol, crémer le beurre avec le sucre à glacer, l'eau, la vanille et l'extrait d'amandes au batteur électrique. À basse vitesse ou à la cuillère de bois, incorporer les ingrédients secs."),
            },
            HowToStep{
                id: 3,
                name: String::from("Sur un plan de travail, déposer la pâte sur une feuille de papier parchemin ou de papier d'aluminium. Former un rouleau d'environ 5cm (2 po) de diamètre. Refermer le rouleau en tortillant les deux extrémités du papier d'aluminium. Réfrigérer environ 3 heures ou jusqu'à ce que la pâte soit très ferme au toucher."),
            },
            HowToStep{
                id: 4,
                name: String::from("Placer la grille au centre du four. Préchauffer le four à 190C (375F). Tapisser deux plaques à biscuits de papier parchemin."),
            },
            HowToStep{
                id: 5,
                name: String::from("Déballer et placer le rouleau sur un plan de travail. Couper en tranches d'environ 1cm (½po) d'épaisseur et les répartir sur les plaques. "),
            },
            HowToStep{
                id: 6,
                name: String::from("Cuire au four, une plaque à la fois, environ 12 minutes, ou jusqu'à ce que les biscuits soient légèrement dorés. Laisser refroidir sur la plaque."),
            },
];

        let recipe_how_to_section_full = vec![RecipeHowToSectionFull {
            id: 1,
            name: String::from(""),
            how_to_steps: recipe_how_to_step,
        }];

        let test_recipe = RecipeFull {
            id: 1,
            name: String::from("Biscuits au beurre réfrigérateur"),
            author: String::from("Ricardocuisine"),
            image: String::from("https://images.ricardocuisine.com/services/recipes/4934.jpg"),
            prep_time: String::from("PT20M"),
            cook_time: String::from("PT12M"),
            total_time: String::from("PT32M"),
            recipe_yield: String::from("40 biscuits, environ"),
            description: String::from("Recette de Ricardo de biscuits au beurre réfrigérateur"),
            categories: test_category,
            keywords: test_keyword,
            ingredients: test_ingredient,
            how_to_section_full: recipe_how_to_section_full,
        };

        assert!(save_recipe(&connection, test_recipe));
    }
}
