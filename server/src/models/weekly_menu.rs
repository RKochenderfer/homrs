use serde::{Serialize, Deserialize};
use chrono::NaiveDate;
use crate::schema::weekly_menus;
use diesel::prelude::*;

#[derive(Deserialize)]
pub struct PostWeeklyMenu {
    pub sunday_meal: i32,
    pub monday_meal: i32,
    pub tuesday_meal: i32,
    pub wednesday_meal: i32,
    pub thursday_meal: i32,
    pub friday_meal: i32,
    pub saturday_meal: i32,
    pub menu_start_date: NaiveDate,
    pub menu_end_date: NaiveDate,
}