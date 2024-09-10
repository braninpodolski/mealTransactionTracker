use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize, Modifier},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    prelude::Alignment,
    Frame,
};

use crate::app::{App, CurrentScreen, ItemInfo};

pub fn ui(frame: &mut Frame, app: &App) {
    // Create the layout sections.
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
            format!("${:.2}", 54.212121).red()
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

// fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
//     // Cut the given rectangle into three vertical pieces
//     let popup_layout = Layout::default()
//         .direction(Direction::Vertical)
//         .constraints([
//             Constraint::Percentage((100 - percent_y) / 2),
//             Constraint::Percentage(percent_y),
//             Constraint::Percentage((100 - percent_y) / 2),
//         ])
//         .split(r);

//     // Then cut the middle vertical piece into three width-wise pieces
//     Layout::default()
//         .direction(Direction::Horizontal)
//         .constraints([
//             Constraint::Percentage((100 - percent_x) / 2),
//             Constraint::Percentage(percent_x),
//             Constraint::Percentage((100 - percent_x) / 2),
//         ])
//         .split(popup_layout[1])[1] // Return the middle chunk
// }