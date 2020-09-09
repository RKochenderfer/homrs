-- This file should undo anything in `up.sql`
ALTER TABLE weekly_menus
    DROP COLUMN sunday_lunch,
    DROP COLUMN sunday_dinner,
    DROP COLUMN monday_lunch,
    DROP COLUMN monday_dinner,
    DROP COLUMN tuesday_lunch,
    DROP COLUMN tuesday_dinner,
    DROP COLUMN wednesday_lunch,
    DROP COLUMN wednesday_dinner,
    DROP COLUMN thursday_lunch,
    DROP COLUMN thursday_dinner,
    DROP COLUMN friday_lunch,
    DROP COLUMN friday_dinner,
    DROP COLUMN saturday_lunch,
    DROP COLUMN saturday_dinner