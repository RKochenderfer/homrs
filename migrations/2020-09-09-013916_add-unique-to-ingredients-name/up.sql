-- Your SQL goes here
ALTER TABLE ingredients ADD CONSTRAINT
    unique_ingredient_name UNIQUE (name)