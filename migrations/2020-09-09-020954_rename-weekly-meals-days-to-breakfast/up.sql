-- Your SQL goes here
ALTER TABLE weekly_menus
    RENAME COLUMN sunday_meal TO sunday_breakfast;

ALTER TABLE weekly_menus
    RENAME COLUMN monday_meal TO monday_breakfast;

ALTER TABLE weekly_menus
    RENAME COLUMN tuesday_meal TO tuesday_breakfast;

ALTER TABLE weekly_menus
    RENAME COLUMN wednesday_meal TO wednesday_breakfast;

ALTER TABLE weekly_menus
    RENAME COLUMN thursday_meal TO thursday_breakfast;

ALTER TABLE weekly_menus
    RENAME COLUMN friday_meal TO friday_breakfast;

ALTER TABLE weekly_menus
    RENAME COLUMN saturday_meal TO saturday_breakfast;
