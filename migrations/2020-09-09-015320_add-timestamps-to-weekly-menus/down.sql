-- This file should undo anything in `up.sql`
ALTER TABLE weekly_menus
    DROP COLUMN created_at,
    DROP COLUMN updated_at,
    DROP COLUMN menu_start_date,
    DROP COLUMN menu_end_date