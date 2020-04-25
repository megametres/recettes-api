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

use crate::database::schema::recipe::dsl::*;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env;

type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

pub struct Values {
    pub db_connection: SqlitePool,
}

lazy_static! {
    pub static ref DB_POOL: Values = {
        Values {
            db_connection: SqlitePool::builder()
                .max_size(8)
                .build(ConnectionManager::new(
                    env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
                ))
                .expect("failed to create db connection_pool"),
        }
    };
    pub static ref DB_TEST_POOL: Values = {
        Values {
            db_connection: SqlitePool::builder()
                .max_size(8)
                .build(ConnectionManager::new("test.db"))
                .expect("failed to create test db connection_pool"),
        }
    };
}

pub fn establish_connection(
) -> diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::SqliteConnection>> {
    dotenv().ok();

    DB_POOL.db_connection.get().unwrap()
}

pub fn load_recipe(recipe_id: i32) -> RecipeFull {
    use crate::database::schema::*;

    let connection = establish_connection();

    let queried_recipe = recipe
        .find(recipe_id)
        .get_result::<Recipe>(&connection)
        .unwrap();

    let queried_category = RecipeCategory::belonging_to(&queried_recipe)
        .inner_join(category::table)
        .select(category::all_columns)
        .load::<Category>(&connection)
        .unwrap();

    let queried_how_to_section = RecipeHowToSection::belonging_to(&queried_recipe)
        .inner_join(how_to_section::table)
        .select(how_to_section::all_columns)
        .load::<HowToSection>(&connection)
        .unwrap();

    let queried_recipe_how_to_section = RecipeHowToSection::belonging_to(&queried_recipe)
        .inner_join(how_to_section::table)
        .select(recipe_how_to_section::all_columns)
        .load::<RecipeHowToSection>(&connection)
        .unwrap();

    let mut queried_how_to_section_full: Vec<RecipeHowToSectionFull> = Vec::new();

    for i in 0..queried_recipe_how_to_section.len() {
        let queried_how_to_step =
            RecipeHowToStep::belonging_to(queried_recipe_how_to_section.get(i).unwrap())
                .inner_join(how_to_step::table)
                .select(how_to_step::all_columns)
                .load::<HowToStep>(&connection)
                .unwrap();
        queried_how_to_section_full.push(RecipeHowToSectionFull {
            id: queried_recipe_how_to_section.get(0).unwrap().id,
            name: queried_how_to_section.get(i).unwrap().name.to_owned(),
            how_to_steps: queried_how_to_step,
        });
    }

    let queried_ingredient = RecipeIngredient::belonging_to(&queried_recipe)
        .inner_join(ingredient::table)
        .select(ingredient::all_columns)
        .load::<Ingredient>(&connection)
        .unwrap();

    let queried_keyword = RecipeKeyword::belonging_to(&queried_recipe)
        .inner_join(keyword::table)
        .select(keyword::all_columns)
        .load::<Keyword>(&connection)
        .unwrap();

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

pub fn save_recipe(recipe_to_save: RecipeFull) -> bool {
    println!("hahaha");
    use super::*;
    use crate::database::schema::category::dsl::*;
    use crate::diesel::associations::HasTable;

    let connection = establish_connection();

    // Inserting the recipe categories

    let ha = NewCategory { name: "Category A" };

    // diesel::insert_into(category::table)
    //     .values(&ha)
    //     .execute(connection)
    //     .unwrap_or_else(|_| panic!("{}", "Error saving categories"));

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

    fn setup_test_db(
    ) -> diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::SqliteConnection>>
    {
        dotenv().ok();
        let connection = DB_TEST_POOL.db_connection.get().unwrap();
        run_pending_migrations(&connection).expect("Fail to run migrations");

        reset_db(&connection);
        connection
    }

    fn reset_db(connection: &SqliteConnection) {
        diesel::delete(recipe_category)
            .execute(connection)
            .expect("could not delete recipe_category associations");
        diesel::delete(category)
            .execute(connection)
            .expect("could not delete category");
        diesel::delete(recipe_keyword)
            .execute(connection)
            .expect("could not delete recipe_keyword associations");
        diesel::delete(keyword)
            .execute(connection)
            .expect("could not delete keyword");
        diesel::delete(recipe_ingredient)
            .execute(connection)
            .expect("could not delete recipe_ingredient associations");
        diesel::delete(ingredient)
            .execute(connection)
            .expect("could not delete ingredient");
        diesel::delete(recipe_how_to_section)
            .execute(connection)
            .expect("could not delete recipe_how_to_section associations");
        diesel::delete(how_to_section)
            .execute(connection)
            .expect("could not delete how_to_section");
        diesel::delete(recipe_how_to_section_how_to_step)
            .execute(connection)
            .expect("could not delete recipe_how_to_section_how_to_step associations");
        diesel::delete(how_to_step)
            .execute(connection)
            .expect("could not delete how_to_step");
        diesel::delete(recipe)
            .execute(connection)
            .expect("could not delete recipe");
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
        use crate::database::schema::*;

        let connection = setup_test_db();

        let test_category = vec![Category {
            id: 1,
            name: "Desserts".to_owned(),
        }];

        let test_ingredient = vec![
            Ingredient {
                id: 1,
                name: r"375 ml (1 1/2 tasse) de farine tout usage non blanchie".to_owned(),
            },
            Ingredient {
                id: 2,
                name: r"250 ml (1 tasse) de poudre d\u2019amandes".to_owned(),
            },
            Ingredient {
                id: 3,
                name: r"5 ml (1 c. \u00e0 th\u00e9) de poudre \u00e0 p\u00e2te".to_owned(),
            },
            Ingredient {
                id: 4,
                name: r"1 ml (1/4 c. \u00e0 th\u00e9) de sel".to_owned(),
            },
            Ingredient {
                id: 5,
                name: r"180 ml (3/4 tasse) de beurre non sal\u00e9, ramolli".to_owned(),
            },
            Ingredient {
                id: 6,
                name: r"250 ml (1 tasse) de sucre \u00e0 glacer".to_owned(),
            },
            Ingredient {
                id: 7,
                name: r"15 ml (1 c. \u00e0 soupe) d\u2019eau froide".to_owned(),
            },
            Ingredient {
                id: 8,
                name: r"5 ml (1 c. \u00e0 th\u00e9) d\u2019extrait de vanille".to_owned(),
            },
            Ingredient {
                id: 9,
                name: r"1 ml (1/4 c. \u00e0 th\u00e9) d\u2019extrait d\u2019amande (facultatif)"
                    .to_owned(),
            },
        ];

        let test_keyword = vec![
            Keyword {
                id: 1,
                name: r"recettes de No\u00ebl".to_owned(),
            },
            Keyword {
                id: 2,
                name: r"desserts de No\u00ebl".to_owned(),
            },
            Keyword {
                id: 3,
                name: r"biscuits de No\u00ebl".to_owned(),
            },
            Keyword {
                id: 4,
                name: r"biscuits sabl\u00e9s au beurre".to_owned(),
            },
            Keyword {
                id: 5,
                name: r"recettes de biscuits sabl\u00e9s au beurre".to_owned(),
            },
            Keyword {
                id: 6,
                name: r"biscuits".to_owned(),
            },
            Keyword {
                id: 7,
                name: r"recettes de biscuits".to_owned(),
            },
        ];

        let recipe_how_to_step = vec![
            HowToStep{
                id: 1,
                name: r"Dans un bol, m\u00e9langer la farine, la poudre d\u2019amandes, la poudre \u00e0 p\u00e2te et le sel. R\u00e9server.".to_owned(),
            },
            HowToStep{
                id: 2,
                name: r"Dans un autre bol, cr\u00e9mer le beurre avec le sucre \u00e0 glacer, l\u2019eau, la vanille et l\u2019extrait d\u2019amandes au batteur \u00e9lectrique. \u00c0 basse vitesse ou \u00e0 la cuill\u00e8re de bois, incorporer les ingr\u00e9dients secs.".to_owned(),
            },
            HowToStep{
                id: 3,
                name: r"Sur un plan de travail, d\u00e9poser la p\u00e2te sur une feuille de papier parchemin ou de papier d\u2019aluminium. Former un rouleau d\u2019environ 5\u00a0cm (2 po) de diam\u00e8tre. Refermer le rouleau en tortillant les deux extr\u00e9mit\u00e9s du papier d\u2019aluminium. R\u00e9frig\u00e9rer environ 3 heures ou jusqu\u2019\u00e0 ce que la p\u00e2te soit tr\u00e8s ferme au toucher.".to_owned(),
            },
            HowToStep{
                id: 4,
                name: r"Placer la grille au centre du four. Pr\u00e9chauffer le four \u00e0 190\u00a0\u00b0C (375 \u00b0F). Tapisser deux plaques \u00e0 biscuits de papier parchemin.".to_owned(),
            },
            HowToStep{
                id: 5,
                name: r"D\u00e9baller et placer le rouleau sur un plan de travail. Couper en tranches d\u2019environ 1 cm (\u00bd po) d\u2019\u00e9paisseur et les r\u00e9partir sur les plaques. ".to_owned(),
            },
            HowToStep{
                id: 6,
                name: r"Cuire au four, une plaque \u00e0 la fois, environ 12 minutes, ou jusqu\u2019\u00e0 ce que les biscuits soient l\u00e9g\u00e8rement dor\u00e9s. Laisser refroidir sur la plaque.".to_owned(),
            },
];

        let recipe_how_to_section_full = vec![RecipeHowToSectionFull {
            id: 1,
            name: "".to_owned(),
            how_to_steps: recipe_how_to_step,
        }];

        let test_recipe = RecipeFull {
            id: 1,
            name: r"Biscuits au beurre r\u00e9frig\u00e9rateur".to_owned(),
            author: r"Ricardocuisine".to_owned(),
            image: r"https://images.ricardocuisine.com/services/recipes/4934.jpg".to_owned(),
            prep_time: r"PT20M".to_owned(),
            cook_time: r"PT12M".to_owned(),
            total_time: r"PT32M".to_owned(),
            recipe_yield: r"40 biscuits, environ".to_owned(),
            description: r"Recette de Ricardo de biscuits au beurre r\u00e9frig\u00e9rateur"
                .to_owned(),
            categories: test_category,
            keywords: test_keyword,
            ingredients: test_ingredient,
            how_to_section_full: recipe_how_to_section_full,
        };

        assert!(save_recipe(test_recipe));
    }
}
