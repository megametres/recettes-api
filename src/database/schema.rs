table! {
    category (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    how_to_section (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    how_to_step (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    ingredient (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    keyword (id) {
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

table! {
    recipe_how_to_section (id) {
        id -> Integer,
        recipe_id -> Integer,
        how_to_section_id -> Integer,
    }
}

table! {
    recipe_how_to_section_how_to_step (id) {
        id -> Integer,
        recipe_how_to_section_id -> Integer,
        how_to_step_id -> Integer,
    }
}

table! {
    recipe_ingredient (id) {
        id -> Integer,
        recipe_id -> Integer,
        ingredient_id -> Integer,
    }
}

table! {
    recipe_keyword (id) {
        id -> Integer,
        recipe_id -> Integer,
        keyword_id -> Integer,
    }
}

joinable!(recipe_category -> category (category_id));
joinable!(recipe_category -> recipe (recipe_id));
joinable!(recipe_how_to_section -> how_to_section (how_to_section_id));
joinable!(recipe_how_to_section -> recipe (recipe_id));
joinable!(recipe_how_to_section_how_to_step -> how_to_step (how_to_step_id));
joinable!(recipe_how_to_section_how_to_step -> recipe_how_to_section (recipe_how_to_section_id));
joinable!(recipe_ingredient -> ingredient (ingredient_id));
joinable!(recipe_ingredient -> recipe (recipe_id));
joinable!(recipe_keyword -> keyword (keyword_id));
joinable!(recipe_keyword -> recipe (recipe_id));

allow_tables_to_appear_in_same_query!(
    category,
    how_to_section,
    how_to_step,
    ingredient,
    keyword,
    recipe,
    recipe_category,
    recipe_how_to_section,
    recipe_how_to_section_how_to_step,
    recipe_ingredient,
    recipe_keyword,
);
