use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    layout::{Constraint, Layout},
    style:: {Stylize, Style, Modifier},
    widgets::{Block, Paragraph},
    text::{Text, Line},
    prelude::{Alignment},
    DefaultTerminal, Frame,
};
use tui_prompts::{TextState, TextPrompt, Prompt};
use chrono::prelude::*;
// use chrono::{DateTime, TimeDelta};

use crate::ui::ui;


pub enum CurrentScreen {
    Main,
    SingleInput,
    RecieptDateInput,
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
    pub single_insert_mode: bool,
    pub currently_editing: Option<ItemInfo>,
    pub current_screen: CurrentScreen,
    pub ingredient_input: String,
    pub price_input: String,
    pub expended_date_input: String,
    pub purchase_date_input: String,
}


impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> App {
        App {
            running: false,
            single_insert_mode: true,
            currently_editing: None,
            current_screen: CurrentScreen::Main,
            ingredient_input: String::new(),
            price_input: String::new(),
            expended_date_input: String::new(),
            purchase_date_input: String::new(),
        }
    }

    pub fn submitIngredient(&mut self) {
        // Send value to database
        
        self.currently_editing = None;
    }

    fn get_monthly_meal_swipe_estimate() -> f64 {
        let today: NaiveDate = Local::now().date_naive();
        let first_of_month: NaiveDate = today.with_day(1).unwrap();

        let days_passed_in_month = today - first_of_month;
        (days_passed_in_month.num_days() as f64) * 15.12 * 2.0
    }
}
