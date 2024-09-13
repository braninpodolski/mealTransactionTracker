use chrono::prelude::*;
use crossterm::ExecutableCommand;
use rusqlite::{params, Connection, Result};
// use chrono::{DateTime, TimeDelta};

pub enum CurrentScreen {
    Main,
    SingleInput,
    RemoveConfirmation,
    EditExpended
}

pub enum ItemInfo {
    Ingredient,
    Price,
    ExpendedDate,
    PurchaseDate,
}

// #[derive(Debug, Default)]
pub struct App{
    /// Is the application running?
    pub running: bool,
    // pub single_insert_mode: bool,
    pub currently_editing: Option<ItemInfo>,
    pub current_screen: CurrentScreen,
    pub ingredient_input: String,
    pub price_input: String,
    pub expended_date_input: String,
    pub purchase_date_input: String,
}

pub struct MealSwipeInfo {
    pub swipes: i64,
    pub cost: f64,
}


impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> App {
        App {
            running: false,
            // single_insert_mode: true,
            currently_editing: None,
            current_screen: CurrentScreen::Main,
            ingredient_input: String::new(),
            price_input: String::new(),
            expended_date_input: String::new(),
            purchase_date_input: String::new(),
        }
    }

    pub fn submit_ingredient(&mut self) {
        // Send value to database
        let mut conn = Connection::open("purchases.db");
        let result = conn.expect("REASON").execute(
            "INSERT INTO purchase (ingredient, price, purchaseDate, expendedDate) VALUES (?1, ?2, ?3, ?4)",
            (&self.ingredient_input, &self.price_input, &self.purchase_date_input, &self.expended_date_input),
        );
        println!("{:?}", result);
    }

    pub fn get_monthly_meal_swipe_estimate() -> f64 {
        let today: NaiveDate = Local::now().date_naive();
        let first_of_month: NaiveDate = today.with_day(1).unwrap();

        let days_passed_in_month = today - first_of_month;
        (days_passed_in_month.num_days() as f64) * 15.12 * 2.0
    }

    pub fn get_semesterly_meal_swipe_estimate() -> MealSwipeInfo {
        let today: NaiveDate = Local::now().date_naive();
        let first_date_semester: NaiveDate = NaiveDate::from_ymd_opt(2024, 9, 1).unwrap();

        let days_passed_in_semester = today - first_date_semester;
        let swipes_used = days_passed_in_semester.num_days() * 2;
        let cost = swipes_used as f64 * 15.12;
        MealSwipeInfo {
            swipes: swipes_used,
            cost: cost,
        }
    }
}
