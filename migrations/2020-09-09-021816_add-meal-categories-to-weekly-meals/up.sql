-- Your SQL goes here
ALTER TABLE weekly_menus
    ADD COLUMN sunday_lunch     INT NOT NULL REFERENCES recipes (id) default 0,
    ADD COLUMN sunday_dinner    INT NOT NULL REFERENCES recipes (id) default 0,
    ADD COLUMN monday_lunch     INT NOT NULL REFERENCES recipes (id) default 0,
    ADD COLUMN monday_dinner    INT NOT NULL REFERENCES recipes (id) default 0,
    ADD COLUMN tuesday_lunch    INT NOT NULL REFERENCES recipes (id) default 0,
    ADD COLUMN tuesday_dinner   INT NOT NULL REFERENCES recipes (id) default 0,
    ADD COLUMN wednesday_lunch  INT NOT NULL REFERENCES recipes (id) default 0,
    ADD COLUMN wednesday_dinner INT NOT NULL REFERENCES recipes (id) default 0,
    ADD COLUMN thursday_lunch   INT NOT NULL REFERENCES recipes (id) default 0,
    ADD COLUMN thursday_dinner  INT NOT NULL REFERENCES recipes (id) default 0,
    ADD COLUMN friday_lunch     INT NOT NULL REFERENCES recipes (id) default 0,
    ADD COLUMN friday_dinner    INT NOT NULL REFERENCES recipes (id) default 0,
    ADD COLUMN saturday_lunch   INT NOT NULL REFERENCES recipes (id) default 0,
    ADD COLUMN saturday_dinner  INT NOT NULL REFERENCES recipes (id) default 0