-- Your SQL goes here
CREATE TABLE IF NOT EXISTS weekly_menus
(
    id             SERIAL PRIMARY KEY,
    sunday_meal    INT NOT NULL REFERENCES recipes (id),
    monday_meal    INT NOT NULL REFERENCES recipes (id),
    tuesday_meal   INT NOT NULL REFERENCES recipes (id),
    wednesday_meal INT NOT NULL REFERENCES recipes (id),
    thursday_meal  INT NOT NULL REFERENCES recipes (id),
    friday_meal    INT NOT NULL REFERENCES recipes (id),
    saturday_meal  INT NOT NULL REFERENCES recipes (id)
)