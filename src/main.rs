use std::{error::Error, io};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};

mod app;
mod ui;
use crate::{
    app::{App, CurrentScreen, ItemInfo},
    ui::ui,
};
use crate::app::ItemInfo::Ingredient;
// fn main() -> color_eyre::Result<()> {
//     color_eyre::install()?;
//     let terminal = ratatui::init();
//     let result = App::new().run(terminal);
//     ratatui::restore();
//     result
// }

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }

            match app.current_screen {
                CurrentScreen::Main => match key.code {
                        KeyCode::Char('q') => {
                            return Ok(false);
                        }
                        KeyCode::Char('i') => {
                            app.current_screen = CurrentScreen::SingleInput;
                            app.currently_editing = Some(ItemInfo::PurchaseDate);
                        }
                        _ => {}
                }
                CurrentScreen::SingleInput => match key.code {
                    KeyCode::Tab => match app.currently_editing {
                        Some(ItemInfo::Ingredient) => {
                            app.currently_editing = Some(ItemInfo::Price);
                        }
                        Some(ItemInfo::Price) => {
                            app.currently_editing = Some(ItemInfo::ExpendedDate);
                        }
                        Some(ItemInfo::ExpendedDate) => {
                            app.currently_editing = Some(ItemInfo::Ingredient);
                        }
                        _ => {}
                    }

                    KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Main;
                        app.currently_editing = None;
                    }
                    KeyCode::Backspace => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                ItemInfo::Ingredient => {
                                    app.ingredient_input.pop();
                                }
                                ItemInfo::Price => {
                                    app.price_input.pop();
                                }
                                ItemInfo::ExpendedDate => {
                                    app.expended_date_input.pop();
                                }
                                ItemInfo::PurchaseDate => {
                                    app.purchase_date_input.pop();
                                }
                            }
                        }
                    }
                    KeyCode::Char(value) => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                ItemInfo::Ingredient => {
                                    app.ingredient_input.push(value);
                                }
                                ItemInfo::Price => {
                                    app.price_input.push(value);
                                }
                                ItemInfo::ExpendedDate => {
                                    app.expended_date_input.push(value);
                                }
                                ItemInfo::PurchaseDate => {
                                    app.purchase_date_input.push(value);
                                }
                            }
                        }
                    }
                    KeyCode::Enter => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                ItemInfo::PurchaseDate => {
                                    if !app.purchase_date_input.is_empty() {
                                        app.currently_editing = Some(ItemInfo::Ingredient);
                                    }
                                }
                                _ => {
                                    if !app.ingredient_input.is_empty() && ! app.price_input.is_empty() {
                                        App::submit_ingredient(app);
                                        app.current_screen = CurrentScreen::SingleInput;
                                        app.currently_editing = Some(Ingredient);
                                        app.ingredient_input.clear();
                                        app.price_input.clear();
                                        app.expended_date_input.clear();
                                    }
                                }
                            }
                        }
                        // implement submission logic

                    }
                    _ => {}
                }
                _ => {}
            };
        }
    }
}
