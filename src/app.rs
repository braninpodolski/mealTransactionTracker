use chrono::Duration;
use chrono::prelude::*;
use crossterm::ExecutableCommand;
use sqlite::Value;

pub enum CurrentScreen {
    Main,
    SingleInput,
    RemoveConfirmation,
    EditExpended
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
pub struct dbResults {
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
        // TODO: Add shorthand for dates ('t" = today, 'y' = yesterday, '-x' = x days ago)
        // Send value to database
        let conn = sqlite::open("src/purchases.db").unwrap();
        let price = (&self.price_input.parse::<f64>().unwrap() * 100.0) as i64;
        let mut expended = "NULL";
        let mut purchase_date: String;


        if !&self.expended_date_input.is_empty() {
            expended = &self.expended_date_input;
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
        statement.bind((4, expended)).unwrap();

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

    // pub fn get_people() -> Result<()> {
    //     let conn = Connection::open("src/purchases.db")?;
    //
    //     let mut stmt = conn.prepare("SELECT rowid, * FROM purchase")?;
    //     let person_iter = stmt.query_map([], |row| {
    //         Ok(dbResults {
    //             id: row.get(0)?,
    //             ingredient: row.get(1)?,
    //             price: row.get(2)?,
    //             expended_date: row.get(3)?,
    //             purchase_date: row.get(4)?,
    //         })
    //     })?;
    //
    //     person_iter
    // }
}
