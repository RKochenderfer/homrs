-- Your SQL goes here
ALTER TABLE weekly_menus
    ADD COLUMN created_at      TIMESTAMP NOT NULL DEFAULT now(),
    ADD COLUMN updated_at      TIMESTAMP NOT NULL DEFAULT now(),
    ADD COLUMN menu_start_date DATE      NOT NULL DEFAULT CURRENT_DATE,
    ADD COLUMN menu_end_date   DATE      NOT NULL DEFAULT CURRENT_DATE
