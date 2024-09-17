use ratatui::{
    layout::{Constraint, Direction, Layout, Rect, Flex},
    style::{Color, Style, Stylize, Modifier},
    text::{Line},
    widgets::{Table, Borders, Block, Row, Paragraph, Clear},
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
        Paragraph::new(" (q) to quit | (i) to add transaction | (e) to edit expended | (r) to remove entry | (R) headless remove").style(Style::new().black().on_blue()),
        navbar_left,
    );

    frame.render_widget(
        Paragraph::new("Sep 8 ").style(Style::new().black().on_blue()).alignment(Alignment::Right),
        navbar_right,
    );

    if let Some(editing) = &app.currently_editing {

        let active_style = Style::default().bg(Color::LightBlue).fg(Color::Black);

        match editing {
            ItemInfo::PurchaseDate => {
                let popup_block = Block::default()
                    .borders(Borders::NONE)
                    .style(Style::default());

                let area = popup_area(frame.area(),30, 6);
                frame.render_widget(Clear, area);
                frame.render_widget(popup_block, area);

                let popup_chunks = Layout::vertical([Constraint::Fill(1)])
                    .margin(1)
                    .split(area);

                let purchase_block = Block::default().title("Purchase Date").borders(Borders::ALL).style(active_style);
                let purchase_text = Paragraph::new(app.purchase_date_input.clone()).block(purchase_block);
                frame.render_widget(purchase_text, popup_chunks[0]);
            }
            _ => {
                let popup_block = Block::default()
                    .borders(Borders::NONE)
                    .style(Style::default());

                let area = popup_area(frame.area(),30, 14);
                frame.render_widget(Clear, area);
                frame.render_widget(popup_block, area);

                let popup_chunks = Layout::vertical([Constraint::Fill(1) ; 3])
                    .margin(1)
                    .split(area);

                let mut ingredient_block = Block::default().title("Ingredient").borders(Borders::ALL);
                let mut price_block = Block::default().title("Price").borders(Borders::ALL);
                let mut expended_block = Block::default().title("Expended Date (Optional)").borders(Borders::ALL);

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

    }
    }

fn popup_area(area: Rect, percent_x: u16, pixel_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Length(pixel_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}