-- This file should undo anything in `up.sql`
ALTER TABLE weekly_menus
    RENAME COLUMN sunday_breakfast TO sunday_meal;

ALTER TABLE weekly_menus
    RENAME COLUMN monday_breakfast TO monday_meal;

ALTER TABLE weekly_menus
    RENAME COLUMN tuesday_breakfast TO tuesday_meal;

ALTER TABLE weekly_menus
    RENAME COLUMN wednesday_breakfast TO wednesday_meal;

ALTER TABLE weekly_menus
    RENAME COLUMN thursday_breakfast TO thursday_meal;

ALTER TABLE weekly_menus
    RENAME COLUMN friday_breakfast TO friday_meal;

ALTER TABLE weekly_menus
    RENAME COLUMN saturday_breakfast TO saturday_meal;