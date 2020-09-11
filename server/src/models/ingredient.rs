use crate::cust_error::Result;
use crate::schema::ingredients;
use diesel::{insert_into, PgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Queryable)]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

impl Ingredient {
    pub fn create(conn: &PgConnection, insert: &InsertIngredient) -> Result<Ingredient> {
        use crate::schema::ingredients::dsl::*;
        Ok(insert_into(ingredients)
            .values(insert)
            .get_result(conn)
            .expect("Failed to insert ingredients"))
    }
}

#[derive(Insertable)]
#[table_name = "ingredients"]
pub struct InsertIngredient {
    pub name: String,
    pub description: Option<String>,
}

impl InsertIngredient {
    pub fn new(post_ingredient: &PostIngredient) -> InsertIngredient {
        InsertIngredient {
            name: post_ingredient.name.to_owned(),
            description: post_ingredient.description.clone(),
        }
    }
}

#[derive(Deserialize)]
pub struct PostIngredient {
    pub name: String,
    pub description: Option<String>,
}

impl PostIngredient {
    /// These are created when selecting ingredients for recipe_step_ingredients. The user will
    /// search for an existing ingredient or will have an option to add new ingredient. This will
    /// then send the post request here to and return the new ingredient to be added to the recipe
    pub fn create_ingredient(&self, conn: &PgConnection) -> Result<Ingredient> {
        let insert = InsertIngredient::new(self);

        Ingredient::create(conn, &insert)
    }
}
