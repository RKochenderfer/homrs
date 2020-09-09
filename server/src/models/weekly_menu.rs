use crate::schema::weekly_menus;
use chrono::NaiveDate;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Queryable)]
pub struct WeeklyMenu {
    pub id: i32,
    pub sunday_breakfast: i32,
    pub monday_breakfast: i32,
    pub tuesday_breakfast: i32,
    pub wednesday_breakfast: i32,
    pub thursday_breakfast: i32,
    pub friday_breakfast: i32,
    pub saturday_breakfast: i32,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
    pub menu_start_date: NaiveDate,
    pub menu_end_date: NaiveDate,
    pub sunday_lunch: i32,
    pub sunday_dinner: i32,
    pub monday_lunch: i32,
    pub monday_dinner: i32,
    pub tuesday_lunch: i32,
    pub tuesday_dinner: i32,
    pub wednesday_lunch: i32,
    pub wednesday_dinner: i32,
    pub thursday_lunch: i32,
    pub thursday_dinner: i32,
    pub friday_lunch: i32,
    pub friday_dinner: i32,
    pub saturday_lunch: i32,
    pub saturday_dinner: i32,
}

#[derive(Deserialize)]
pub struct PostWeeklyMenu {
    pub sunday_breakfast: i32,
    pub monday_breakfast: i32,
    pub tuesday_breakfast: i32,
    pub wednesday_breakfast: i32,
    pub thursday_breakfast: i32,
    pub friday_breakfast: i32,
    pub saturday_breakfast: i32,
    pub sunday_lunch: i32,
    pub sunday_dinner: i32,
    pub monday_lunch: i32,
    pub monday_dinner: i32,
    pub tuesday_lunch: i32,
    pub tuesday_dinner: i32,
    pub wednesday_lunch: i32,
    pub wednesday_dinner: i32,
    pub thursday_lunch: i32,
    pub thursday_dinner: i32,
    pub friday_lunch: i32,
    pub friday_dinner: i32,
    pub saturday_lunch: i32,
    pub saturday_dinner: i32,
    pub menu_start_date: NaiveDate,
    pub menu_end_date: NaiveDate,
}
