use chrono::prelude::*;
use chrono::Duration;

use ratatui::style::{Color};
use ratatui::widgets::{ScrollbarState, TableState};
use sqlite::State;

pub enum CurrentScreen {
    Main,
    SingleInput,
    RemoveConfirmation,
    EditExpended,
    Query
}

pub enum ItemInfo {
    Id,
    Ingredient,
    Price,
    ExpendedDate,
    PurchaseDate,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct DbResults {
    id: i32,
    ingredient: String,
    price: i32,
    expended_date: String,
    purchase_date: String,
}

// #[derive(Debug, Default)]
pub struct App{
    /// Is the application running?
    pub running: bool,
    pub order_by: String,
    pub search_param: String,
    pub state: TableState,
    pub item_count: i32,
    pub row_data: Vec<Vec<String>>,
    // pub single_insert_mode: bool,
    pub currently_editing: Option<ItemInfo>,
    pub current_screen: CurrentScreen,
    pub ingredient_input: String,
    pub price_input: String,
    pub expended_date_input: String,
    pub query_input: String,
    pub purchase_date_input: String,
    pub scroll_state: ScrollbarState
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
            order_by: "expendedDate DESC, purchaseDate ASC".to_string(),
            search_param: "true".to_string(),
            state: TableState::default().with_selected(0),
            item_count: 0,
            row_data: Vec::<Vec<String>>::new(),
            // single_insert_mode: true,
            currently_editing: None,
            scroll_state: ScrollbarState::new(1),
            current_screen: CurrentScreen::Main,
            ingredient_input: String::new(),
            price_input: String::new(),
            query_input: String::new(),
            expended_date_input: String::new(),
            purchase_date_input: String::new(),
        }
    }

    pub fn submit_ingredient(&mut self) {
        // Send value to database
        let conn = sqlite::open("src/purchases.db").unwrap();
        let price = (&self.price_input.parse::<f64>().unwrap() * 100.0) as i64;
        let mut expended = String::from("NULL");
        let mut purchase_date: String;


        if !&self.expended_date_input.is_empty() {
            if self.expended_date_input.to_lowercase().eq("t") {
                let today = Local::now();
                expended = format!("{}",today.format("%Y-%m-%d"));
            } else if self.expended_date_input.to_lowercase().eq("y") {
                let yesterday = Local::now() - Duration::days(1);
                expended = format!("{}", yesterday.format("%Y-%m-%d"));
            } else {
                expended = self.expended_date_input.clone();
            }
        }

        if self.purchase_date_input.to_lowercase().eq("t") {
            let today = Local::now();
            purchase_date = format!("{}",today.format("%Y-%m-%d"));
        } else if self.purchase_date_input.to_lowercase().eq("y") {
            let yesterday = Local::now() - Duration::days(1);
            purchase_date = format!("{}", yesterday.format("%Y-%m-%d"));
        } else {
            purchase_date = self.purchase_date_input.clone();
        }




        let query = "INSERT INTO purchase (ingredient, price, purchaseDate, expendedDate) VALUES (?, ?, ?, ?)";
        let mut statement = conn.prepare(query).unwrap();

        statement.bind((1, self.ingredient_input.as_str())).unwrap();
        statement.bind((2, price)).unwrap();
        statement.bind((3, purchase_date.as_str())).unwrap();
        statement.bind((4, expended.as_str())).unwrap();

        statement.next().unwrap();
    }

    pub fn update_expended(item_id: String, new_date: String) {
        let conn = sqlite::open("src/purchases.db").unwrap();
        let query = "UPDATE purchase SET expendedDate = ? WHERE rowid = ?";

        let mut statement = conn.prepare(query).unwrap();

        let _ = statement.bind((1, new_date.as_str()));
        let _ = statement.bind((2, item_id.as_str()));

        statement.next().unwrap();
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

    // Queries SQLite and returns Table Row Vector
    pub fn get_ingredient_entries(&mut self) -> Vec<Vec<String>> {
        let conn = sqlite::open("src/purchases.db").unwrap();

        let query = format!("SELECT rowid, * FROM purchase WHERE {} ORDER BY {}", &self.search_param, &self.order_by);
        let mut rows = Vec::<Vec<String>>::new();

        let mut statement = conn.prepare(query).unwrap();
        let mut i = 0;

        while let State::Row = statement.next().unwrap() {
            // Use Option<String> to handle possible NULL values
            let rowid = statement.read::<String, _>("rowid").unwrap_or_else(|_| "Unknown".to_string());
            let ingredient = statement.read::<String, _>("ingredient").unwrap_or_else(|_| "Unknown".to_string());
            let price = format!("${:.2}",(statement.read::<i64, _>("price").unwrap_or_else(|_| 0) as f64) / 100.00);
            let purchase_date = statement.read::<String, _>("purchaseDate").unwrap_or_else(|_| "Unknown".to_string());
            let expended_date = statement.read::<String, _>("expendedDate").unwrap_or_else(|_| "Unknown".to_string());

            rows.push(vec![
                rowid,
                ingredient,
                price,
                purchase_date,
                expended_date,
            ]);

            i += 1;
        }
        self.item_count = i;

        rows
    }

    // Table Navigation Functions
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= (self.item_count - 1) as usize {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i);
    }
    pub fn prev(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.item_count as usize - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i);
    }
}
