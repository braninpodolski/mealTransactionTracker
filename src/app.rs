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


#[derive(Debug, Default)]
pub struct App{
    /// Is the application running?
    running: bool,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.draw_ui(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn draw_ui(&mut self, frame: &mut Frame) {
        
        // Basic layout
        let [header_area, main_area, navbar_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .areas(frame.area());

        // Navbar layout
        let [navbar_left, navbar_right] = Layout::horizontal([
            Constraint::Min(10),
            Constraint::Length(25),
        ])
        .areas(navbar_area);

        // Divide layout in top/bottom
        let [top_half, bottom_half] = Layout::vertical([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .areas(main_area);

        // Top half layout
        let [top_left, top_right] = Layout::horizontal([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .areas(top_half);

        let monthly_text = Text::from(vec![
            Line::from(vec![
                "Meal Swipe Bill: ".into(),
                format!("${:.2}", App::get_monthly_meal_swipe_estimate()).red()
            ])
        ]);
        

        frame.render_widget(
            Paragraph::new("Meal Price Tracker")
                .style(Style::new().black().on_blue())
                .centered(),
            header_area,
        );
        
        frame.render_widget(
            Paragraph::new(monthly_text)
                .block(Block::bordered().title("Statistics\n"))
                .centered()
                .add_modifier(Modifier::BOLD)
                .blue(),
            top_left,
        );
        
        

        frame.render_widget(
            Paragraph::new("")
                .block(Block::bordered().title("Daily Spending (Month)"))
                .centered()
                .add_modifier(Modifier::BOLD)
                .blue(),
            top_right,
        );

        frame.render_widget(
            Paragraph::new("")
                .block(Block::bordered().title("Transactions"))
                .centered()
                .add_modifier(Modifier::BOLD)
                .blue(),
            bottom_half,
        );

        frame.render_widget(
            Paragraph::new(" (q) to quit | (i) for single entry | (I) for multi entry | (e) to edit expended | (r) to remove entry | (R) headless remove").style(Style::new().black().on_blue()),
            navbar_left,
        );

        frame.render_widget(
            Paragraph::new("Sep 8 ").style(Style::new().black().on_blue()).alignment(Alignment::Right),
            navbar_right,
        );
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            // Add other key handlers here.
            _ => {}
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }

    fn get_monthly_meal_swipe_estimate() -> f64 {
        let today: NaiveDate = Local::now().date_naive();
        let first_of_month: NaiveDate = today.with_day(1).unwrap();

        let days_passed_in_month = today - first_of_month;
        (days_passed_in_month.num_days() as f64) * 15.12 * 2.0
    }
}
