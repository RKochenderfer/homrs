-- Your SQL goes here
ALTER TABLE weekly_menus
    ADD COLUMN sunday_lunch     INT REFERENCES recipes (id),
    ADD COLUMN sunday_dinner    INT REFERENCES recipes (id),
    ADD COLUMN monday_lunch     INT REFERENCES recipes (id),
    ADD COLUMN monday_dinner    INT REFERENCES recipes (id),
    ADD COLUMN tuesday_lunch    INT REFERENCES recipes (id),
    ADD COLUMN tuesday_dinner   INT REFERENCES recipes (id),
    ADD COLUMN wednesday_lunch  INT REFERENCES recipes (id),
    ADD COLUMN wednesday_dinner INT REFERENCES recipes (id),
    ADD COLUMN thursday_lunch   INT REFERENCES recipes (id),
    ADD COLUMN thursday_dinner  INT REFERENCES recipes (id),
    ADD COLUMN friday_lunch     INT REFERENCES recipes (id),
    ADD COLUMN friday_dinner    INT REFERENCES recipes (id),
    ADD COLUMN saturday_lunch   INT REFERENCES recipes (id),
    ADD COLUMN saturday_dinner  INT REFERENCES recipes (id)