-- This file should undo anything in `up.sql`
ALTER TABLE recipe_step_ingredients
    RENAME COLUMN step_id TO step_number;