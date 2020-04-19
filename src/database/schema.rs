table! {
    category (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    recipe (id) {
        id -> Integer,
        name -> Text,
        author -> Text,
        image -> Text,
        prep_time -> Text,
        cook_time -> Text,
        total_time -> Text,
        recipe_yield -> Text,
        description -> Text,
        json_ld -> Text,
    }
}

table! {
    recipe_category (id) {
        id -> Integer,
        recipe_id -> Integer,
        category_id -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    category,
    recipe,
    recipe_category,
);
