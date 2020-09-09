-- Your SQL goes here
CREATE TABLE IF NOT EXISTS recipes
(
    id                   SERIAL PRIMARY KEY,
    recipe_name          VARCHAR   NOT NULL,
    recipe_description   VARCHAR,
    meal_category        VARCHAR,
    preparation_time     VARCHAR,
    number_of_servings   INT,
    calories_per_serving INT,
    user_id              INT REFERENCES users (id),
    created_at           TIMESTAMP NOT NULL,
    updated_at           TIMESTAMP NOT NULL
)