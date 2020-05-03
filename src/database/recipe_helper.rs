extern crate diesel_migrations;
extern crate dotenv;

use super::models::model_category::*;
use super::models::model_how_to_section::*;
use super::models::model_how_to_step::*;
use super::models::model_ingredient::*;
use super::models::model_keyword::*;
use super::models::model_recipe::*;

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

fn print_type_of<T>(_: &T) -> &str {
    std::any::type_name::<T>()
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

macro_rules! upsert_recipe_elements {
    ($connection:expr, $table_name:ident, $model:ident, $elements:expr) => {{
        let mut return_elements = Vec::new();
        for x in $elements {
            let inserted_element: $model = diesel::insert_into($table_name::table)
                .values($table_name::name.eq(x.name.to_owned()))
                .on_conflict($table_name::name)
                .do_update()
                .set($table_name::name.eq(excluded($table_name::name)))
                .get_result::<$model>($connection)
                .unwrap();
            return_elements.push(inserted_element.id);
        }
        return_elements
    }};
}

macro_rules! link_recipe_elements {
    ($connection:expr, $table_name:ident, $parent_column:ident, $child_column:ident, $insertion_model:ident, $elements:expr, $parent_id:expr) => {{
        for x in $elements {
            let element_to_insert = $insertion_model {
                $parent_column: $parent_id,
                $child_column: x,
            };
            diesel::insert_into($table_name::table)
                .values(&element_to_insert)
                .execute($connection)
                .unwrap_or_else(|_| {
                    panic!(
                        "{} {:?} ({},{})",
                        "Error saving new ",
                        print_type_of(&element_to_insert),
                        $parent_id,
                        x
                    )
                });
        }
    }};
}

macro_rules! link_recipe_elements_and_return {
    ($connection:expr, $table_name:ident, $parent_column:ident, $child_column:ident, $return_model:ident, $insertion_model:ident, $elements:expr, $parent_id:expr) => {{
        let mut return_elements: Vec<i32> = Vec::new();
        for x in $elements {
            let element_to_insert = $insertion_model {
                $parent_column: $parent_id,
                $child_column: x,
            };
            let inserted_element: $return_model = diesel::insert_into($table_name::table)
                .values(&element_to_insert)
                .get_result($connection)
                .unwrap_or_else(|_| {
                    panic!(
                        "{} {:?} ({},{})",
                        "Error saving new ",
                        print_type_of(&element_to_insert),
                        $parent_id,
                        x
                    )
                });
            return_elements.push(inserted_element.id);
        }
        return_elements
    }};
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
        categories: Some(queried_category),
        keywords: Some(queried_keyword),
        ingredients: Some(queried_ingredient),
        how_to_section_full: Some(queried_how_to_section_full),
        json_ld: queried_recipe.json_ld,
    }
}

pub fn get_recipe_list(connection: &PgConnection) -> Vec<RecipeSimple> {
    let mut return_recipe_list: Vec<RecipeSimple> = Vec::new();
    let recipe_list: Vec<Recipe> = recipe
        .load::<Recipe>(connection)
        .expect("Error loading posts");
    for x in recipe_list {
        return_recipe_list.push(RecipeSimple {
            id: x.id,
            name: x.name,
        });
    }
    return_recipe_list
}

pub fn save_recipe(connection: &PgConnection, recipe_to_save: &RecipeFull) -> bool {
    use super::schema::*;
    use diesel::pg::upsert::excluded;

    // TODO :: implement a transaction

    // Insertion in table recipe
    let recipe_to_insert = NewRecipe {
        name: &recipe_to_save.name,
        author: Some(&recipe_to_save.author.as_ref().unwrap()),
        image: Some(&recipe_to_save.image.as_ref().unwrap()),
        prep_time: Some(&recipe_to_save.prep_time.as_ref().unwrap()),
        cook_time: Some(&recipe_to_save.cook_time.as_ref().unwrap()),
        total_time: Some(&recipe_to_save.total_time.as_ref().unwrap()),
        recipe_yield: Some(&recipe_to_save.recipe_yield.as_ref().unwrap()),
        description: &recipe_to_save.description,
        json_ld: &recipe_to_save.json_ld,
    };

    let inserted_recipe: Vec<Recipe> = diesel::insert_into(recipe::table)
        .values(&recipe_to_insert)
        .get_results(connection)
        .unwrap_or_else(|_| panic!("{}{}", "Error saving new recipe ", recipe_to_insert.name));

    // Insertion in table category
    let inserted_categories = upsert_recipe_elements!(
        connection,
        category,
        Category,
        recipe_to_save.categories.as_ref().unwrap()
    );

    // Insertion in table recipe_category
    link_recipe_elements!(
        connection,
        recipe_category,
        recipe_id,
        category_id,
        NewRecipeCategory,
        inserted_categories,
        inserted_recipe.get(0).unwrap().id
    );

    // Insertion in table keyword
    let inserted_keywords = upsert_recipe_elements!(
        connection,
        keyword,
        Keyword,
        recipe_to_save.keywords.as_ref().unwrap()
    );

    // Insertion in table recipe_keyword
    link_recipe_elements!(
        connection,
        recipe_keyword,
        recipe_id,
        keyword_id,
        NewRecipeKeyword,
        inserted_keywords,
        inserted_recipe.get(0).unwrap().id
    );

    // Insertion in table ingredient
    let inserted_ingredients = upsert_recipe_elements!(
        connection,
        ingredient,
        Ingredient,
        recipe_to_save.ingredients.as_ref().unwrap()
    );

    // Insertion in table recipe_ingredient
    link_recipe_elements!(
        connection,
        recipe_ingredient,
        recipe_id,
        ingredient_id,
        NewRecipeIngredient,
        inserted_ingredients,
        inserted_recipe.get(0).unwrap().id
    );

    // Insertion in table how_to_section
    let inserted_how_to_sections = upsert_recipe_elements!(
        connection,
        how_to_section,
        HowToSection,
        recipe_to_save.how_to_section_full.as_ref().unwrap()
    );

    // Insertion in table recipe_how_to_section
    let inserted_recipe_how_to_sections = link_recipe_elements_and_return!(
        connection,
        recipe_how_to_section,
        recipe_id,
        how_to_section_id,
        RecipeHowToSection,
        NewRecipeHowToSection,
        inserted_how_to_sections,
        inserted_recipe.get(0).unwrap().id
    );

    // Insertion in table how_to_step and recipe_how_to_section_how_to_step
    let how_to_section_full = recipe_to_save.how_to_section_full.as_ref().unwrap();
    for x in 0..inserted_recipe_how_to_sections.len() {
        let test: &Vec<HowToStep> = how_to_section_full[0].how_to_steps.as_ref();
        let inserted_how_to_steps =
            upsert_recipe_elements!(connection, how_to_step, HowToStep, test);

        link_recipe_elements!(
            connection,
            recipe_how_to_section_how_to_step,
            recipe_how_to_section_id,
            how_to_step_id,
            NewRecipeHowToStep,
            inserted_how_to_steps,
            *inserted_recipe_how_to_sections.get(x).unwrap()
        );
    }

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

    macro_rules! empty_recipe_table {
        ($connection:expr, $table:expr) => {
            diesel::delete($table)
                .execute($connection)
                .unwrap_or_else(|_| {
                    panic!("{} {:?} ", "Error deleting table ", print_type_of(&$table))
                });
        };
    }

    fn setup_test_db() -> PgConnection {
        dotenv().ok();

        let database_test_url =
            env::var("DATABASE_TEST_URL").expect("DATABASE_TEST_URL must be set");

        let connection = PgConnection::establish(&database_test_url).unwrap();

        run_pending_migrations(&connection).expect("Fail to run migrations");
        reset_db(&connection);
        connection
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
            author: Some("Recipe A author"),
            image: Some("Recipe A image"),
            prep_time: Some("Recipe A prep_time"),
            cook_time: Some("Recipe A cook_time"),
            total_time: Some("Recipe A total_time"),
            recipe_yield: Some("Recipe A recipe_yield"),
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
        assert_eq!(
            test_recipe_a.author.unwrap(),
            result.get(0).unwrap().author.as_ref().unwrap()
        );
        assert_eq!(
            test_recipe_a.image.unwrap(),
            result.get(0).unwrap().image.as_ref().unwrap()
        );
        assert_eq!(
            test_recipe_a.prep_time.unwrap(),
            result.get(0).unwrap().prep_time.as_ref().unwrap()
        );
        assert_eq!(
            test_recipe_a.cook_time.unwrap(),
            result.get(0).unwrap().cook_time.as_ref().unwrap()
        );
        assert_eq!(
            test_recipe_a.total_time.unwrap(),
            result.get(0).unwrap().total_time.as_ref().unwrap()
        );
        assert_eq!(
            test_recipe_a.recipe_yield.unwrap(),
            result.get(0).unwrap().recipe_yield.as_ref().unwrap()
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
            author: Some(String::from("Ricardocuisine")),
            image: Some(String::from(
                "https://images.ricardocuisine.com/services/recipes/4934.jpg",
            )),
            prep_time: Some(String::from("PT20M")),
            cook_time: Some(String::from("PT12M")),
            total_time: Some(String::from("PT32M")),
            recipe_yield: Some(String::from("40 biscuits, environ")),
            description: String::from("Recette de Ricardo de biscuits au beurre réfrigérateur"),
            categories: Some(test_category),
            keywords: Some(test_keyword),
            ingredients: Some(test_ingredient),
            how_to_section_full: Some(recipe_how_to_section_full),
            json_ld: String::from("blablabla"),
        };

        assert!(save_recipe(&connection, &test_recipe));
        assert!(save_recipe(&connection, &test_recipe));
    }
}
