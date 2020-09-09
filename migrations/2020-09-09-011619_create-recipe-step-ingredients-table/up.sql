-- Your SQL goes here
CREATE TABLE IF NOT EXISTS recipe_step_ingredients (
    id SERIAL PRIMARY KEY,
    step_number INT NOT NULL REFERENCES recipe_steps(id),
    ingredient_id INT NOT NULL REFERENCES ingredients(id)
)