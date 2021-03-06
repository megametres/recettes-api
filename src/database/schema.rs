table! {
    category (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    how_to_section (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    how_to_step (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    ingredient (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    keyword (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    recipe (id) {
        id -> Int4,
        name -> Varchar,
        category_id -> Int4,
        author -> Nullable<Varchar>,
        image -> Nullable<Text>,
        prep_time -> Nullable<Varchar>,
        cook_time -> Nullable<Varchar>,
        total_time -> Nullable<Varchar>,
        recipe_yield -> Nullable<Varchar>,
        description -> Text,
        json_ld -> Nullable<Text>,
        source -> Nullable<Varchar>,
    }
}

table! {
    recipe_how_to_section (id) {
        id -> Int4,
        recipe_id -> Int4,
        how_to_section_id -> Int4,
    }
}

table! {
    recipe_how_to_section_how_to_step (id) {
        id -> Int4,
        recipe_how_to_section_id -> Int4,
        how_to_step_id -> Int4,
    }
}

table! {
    recipe_ingredient (id) {
        id -> Int4,
        recipe_id -> Int4,
        ingredient_id -> Int4,
    }
}

table! {
    recipe_keyword (id) {
        id -> Int4,
        recipe_id -> Int4,
        keyword_id -> Int4,
    }
}

joinable!(recipe -> category (category_id));
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
    recipe_how_to_section,
    recipe_how_to_section_how_to_step,
    recipe_ingredient,
    recipe_keyword,
);
