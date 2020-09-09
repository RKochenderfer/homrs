table! {
    ingredients (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
    }
}

table! {
    recipe_step_ingredients (id) {
        id -> Int4,
        step_number -> Int4,
        ingredient_id -> Int4,
    }
}

table! {
    recipe_steps (id) {
        id -> Int4,
        step_number -> Int4,
        instructions -> Varchar,
        recipe_id -> Int4,
    }
}

table! {
    recipes (id) {
        id -> Int4,
        recipe_name -> Varchar,
        recipe_description -> Nullable<Varchar>,
        meal_category -> Nullable<Varchar>,
        preparation_time -> Nullable<Varchar>,
        number_of_servings -> Nullable<Int4>,
        calories_per_serving -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    sessions (id) {
        id -> Int4,
        user_id -> Int4,
        token -> Varchar,
        last_action -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        password_hash -> Varchar,
        user_role -> Bpchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    weekly_menus (id) {
        id -> Int4,
        sunday_breakfast -> Int4,
        monday_breakfast -> Int4,
        tuesday_breakfast -> Int4,
        wednesday_breakfast -> Int4,
        thursday_breakfast -> Int4,
        friday_breakfast -> Int4,
        saturday_breakfast -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        menu_start_date -> Date,
        menu_end_date -> Date,
        sunday_lunch -> Nullable<Int4>,
        sunday_dinner -> Nullable<Int4>,
        monday_lunch -> Nullable<Int4>,
        monday_dinner -> Nullable<Int4>,
        tuesday_lunch -> Nullable<Int4>,
        tuesday_dinner -> Nullable<Int4>,
        wednesday_lunch -> Nullable<Int4>,
        wednesday_dinner -> Nullable<Int4>,
        thursday_lunch -> Nullable<Int4>,
        thursday_dinner -> Nullable<Int4>,
        friday_lunch -> Nullable<Int4>,
        friday_dinner -> Nullable<Int4>,
        saturday_lunch -> Nullable<Int4>,
        saturday_dinner -> Nullable<Int4>,
    }
}

joinable!(recipe_step_ingredients -> ingredients (ingredient_id));
joinable!(recipe_step_ingredients -> recipe_steps (step_number));
joinable!(recipe_steps -> recipes (recipe_id));
joinable!(recipes -> users (user_id));
joinable!(sessions -> users (user_id));

allow_tables_to_appear_in_same_query!(
    ingredients,
    recipe_step_ingredients,
    recipe_steps,
    recipes,
    sessions,
    users,
    weekly_menus,
);
