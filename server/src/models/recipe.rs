use crate::cust_error::Result;
use crate::models::recipe_step::PostRecipeStep;
use crate::schema::recipes;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::{insert_into, PgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Queryable, AsChangeset)]
#[table_name = "recipes"]
pub struct Recipe {
    id: i32,
    recipe_name: String,
    recipe_description: Option<String>,
    meal_category: Option<String>,
    preparation_time: Option<String>,
    number_of_servings: Option<i32>,
    calories_per_serving: Option<i32>,
    user_id: Option<i32>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl Recipe {
    pub fn create(conn: &PgConnection, insert: &InsertRecipe) -> Result<Recipe> {
        use crate::schema::recipes::dsl::*;

        Ok(insert_into(recipes)
            .values(insert)
            .get_result(conn)
            .expect("Failed to insert recipes."))
    }
}

#[derive(Insertable)]
#[table_name = "recipes"]
pub struct InsertRecipe {
    recipe_name: String,
    recipe_description: Option<String>,
    meal_category: Option<String>,
    preparation_time: Option<String>,
    number_of_servings: Option<i32>,
    calories_per_serving: Option<i32>,
    user_id: Option<i32>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl InsertRecipe {
    pub fn new(r: &PostRecipe, recipe_user_id: i32) -> InsertRecipe {
        InsertRecipe {
            recipe_name: r.recipe_name.clone(),
            recipe_description: r.recipe_description.clone(),
            meal_category: r.meal_category.clone(),
            preparation_time: r.preparation_time.clone(),
            number_of_servings: r.number_of_servings.clone(),
            calories_per_serving: r.number_of_servings.clone(),
            user_id: Some(recipe_user_id),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Deserialize)]
pub struct PostRecipe {
    pub recipe_name: String,
    pub recipe_description: Option<String>,
    pub meal_category: Option<String>,
    pub preparation_time: Option<String>,
    pub number_of_servings: Option<i32>,
    pub calories_per_serving: Option<i32>,
    pub recipe_steps: Vec<PostRecipeStep>,
}

impl PostRecipe {
    /// Handles the creation of recipe, recipe_step, recipe_step_ingredient
    pub fn create_recipe(&self, conn: &PgConnection, recipe_user_id: i32) -> Result<()> {
        // Create transaction for whole recipe
        let insert_recipe = InsertRecipe::new(self, recipe_user_id);
        // Create the entry in the recipes table
        conn.transaction::<_, DieselError, _>(|| {
            let recipe = Recipe::create(conn, &insert_recipe).expect("Failed to insert recipe");
            for step in self.recipe_steps.iter() {
                let recipe_step = step
                    .create_recipe_step(conn, recipe.id)
                    .expect("Failed to insert recipe_step");
                for ingredient_step in step.ingredients.iter() {
                    ingredient_step
                        .create_recipe_step_ingredient(conn, recipe_step.id)
                        .expect("Failed to insert ingredient step.");
                }
            }

            Ok(())
        })
        .expect("Failed to create recipe");

        // Create recipe_steps and recipe_steps_ingredients together

        Ok(())
    }
}
