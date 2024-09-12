use chrono::format::Item;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize, Modifier},
    text::{Line},
    widgets::*,
    prelude::*,
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

    let stats_block = Block::bordered().title("Statistics\n");
    let graph_block = Block::bordered().title("Daily Spending (Month)");
    let table_block = Block::bordered().title("Transactions");

    let monthly_text:Line = vec![
            "Month Meal Swipe Bill: ".into(),
            format!("${:.2}", App::get_monthly_meal_swipe_estimate()).red()
     ].into();

    let semester_cost_text:Line = vec![
            "Semester Meal Swipe Bill: ".into(),
            format!("${:.2}", App::get_semesterly_meal_swipe_estimate().cost).red()
        ].into();

    let semester_count_text:Line = vec![
        "Semester Meal Swipes Used: ".into(),
        format!("{}/210", App::get_semesterly_meal_swipe_estimate().swipes).red()
    ].into();

    let stat_text = vec![
        monthly_text,
        semester_cost_text,
        semester_count_text,
    ];

    frame.render_widget(
        Paragraph::new("Meal Price Tracker")
            .style(Style::new().black().on_blue())
            .centered(),
        header_area,
    );

    frame.render_widget(
        Paragraph::new(stat_text)
            .block(stats_block)
            .centered()
            .add_modifier(Modifier::BOLD)
            .blue(),
        top_left,
    );

    frame.render_widget(
        Paragraph::new("")
            .block(graph_block)
            .centered()
            .add_modifier(Modifier::BOLD)
            .blue(),
        top_right,
    );

    frame.render_widget(
        Paragraph::new("")
            .block(table_block)
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

    if let Some(editing) = &app.currently_editing {
        let popup_block = Block::default()
            .title("Enter a new ingredient")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let area = centered_rect(30, 30, frame.area());
        frame.render_widget(Clear, area);
        frame.render_widget(popup_block, area);

        let popup_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Ratio(1, 3) ; 3])
            .split(area);

        let mut ingredient_block = Block::default().title("Ingredient").borders(Borders::ALL);
        let mut price_block = Block::default().title("Price").borders(Borders::ALL);
        let mut expended_block = Block::default().title("Expended Date (Optional)").borders(Borders::ALL);


        let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);

        match editing {
            ItemInfo::Ingredient => ingredient_block = ingredient_block.style(active_style),
            ItemInfo::Price => price_block = price_block.style(active_style),
            ItemInfo::ExpendedDate => expended_block = expended_block.style(active_style),
            _ => {}
        };

        let ingredient_text = Paragraph::new(app.ingredient_input.clone()).block(ingredient_block);
        frame.render_widget(ingredient_text, popup_chunks[0]);

        let price_text = Paragraph::new(app.price_input.clone()).block(price_block);
        frame.render_widget(price_text, popup_chunks[1]);

        let expended_text = Paragraph::new(app.expended_date_input.clone()).block(expended_block);
        frame.render_widget(expended_text, popup_chunks[2]);
    }
    }

    fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        // Cut the given rectangle into three vertical pieces
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(r);

        // Then cut the middle vertical piece into three width-wise pieces
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1])[1] // Return the middle chunk
    }