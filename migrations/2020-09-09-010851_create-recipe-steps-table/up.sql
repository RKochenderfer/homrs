-- Your SQL goes here
CREATE TABLE IF NOT EXISTS recipe_steps
(
    id           SERIAL PRIMARY KEY,
    step_number  INT     NOT NULL,
    instructions VARCHAR NOT NULL,
    recipe_id    INT     NOT NULL REFERENCES recipes (id)
)