// src/ui/tui.rs
use ratatui::{
    Frame,
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
 };
 
 use crate::models::estajo::Sex;
 use super::app::App;
 
 fn draw_population(f: &mut Frame, app: &App, area: Rect) {
    let total = app.world.estajoj.len();
    let males = app.world.estajoj.values()
        .filter(|e| matches!(e.sex, Sex::Male))
        .count();
    let females = total - males;
 
    let text = Text::from(vec![
        Line::from(vec![Span::raw(format!("Total: {}", total))]),
        Line::from(vec![Span::raw(format!("Males: {}", males))]),
        Line::from(vec![Span::raw(format!("Females: {}", females))]),
    ]);
 
    let block = Block::default()
        .title("Population")
        .borders(Borders::ALL);
    let paragraph = Paragraph::new(text)
        .block(block)
        .style(Style::default().fg(Color::White));
    f.render_widget(paragraph, area);
 }
 
 fn draw_events_log(f: &mut Frame, app: &App, area: Rect) {
    let events: Vec<ListItem> = app.world
        .get_recent_events(10)
        .iter()
        .map(|e| ListItem::new(format!("{}", e)))
        .collect();
 
    let block = Block::default()
        .title("Events Log")
        .borders(Borders::ALL);
    let list = List::new(events)
        .block(block)
        .style(Style::default().fg(Color::White));
    f.render_widget(list, area);
 }
 
 fn draw_needs_status(f: &mut Frame, app: &App, area: Rect) {
    let hungry_count = app.world.estajoj.values()
        .filter(|e| e.needs.hunger < 30.0)
        .count();
    let ambitious_count = app.world.estajoj.values()
        .filter(|e| e.needs.ambition > 70.0)
        .count();
 
    let text = Text::from(vec![
        Line::from(vec![Span::raw(format!("Hungry: {}", hungry_count))]),
        Line::from(vec![Span::raw(format!("Ambitious: {}", ambitious_count))]),
    ]);
 
    let block = Block::default()
        .title("Needs Status")
        .borders(Borders::ALL);
    let paragraph = Paragraph::new(text)
        .block(block)
        .style(Style::default().fg(Color::White));
    f.render_widget(paragraph, area);
 }
 
 fn draw_selected_estajo(f: &mut Frame, app: &App, area: Rect) {
    let text = if let Some(id) = app.selected_estajo_id {
        if let Some(estajo) = app.world.estajoj.get(&id) {
            Text::from(vec![
                Line::from(vec![Span::raw(format!("Name: {}", estajo.name))]),
                Line::from(vec![Span::raw(format!("Sex: {:?}", estajo.sex))]),
                Line::from(vec![Span::raw(format!("Life: {:.1}%", estajo.life))]),
                Line::from(vec![Span::raw(format!("Hunger: {:.1}%", estajo.needs.hunger))]),
                Line::from(vec![Span::raw(format!("Ambition: {:.1}%", estajo.needs.ambition))]),
            ])
        } else {
            Text::from(vec![Line::from(vec![Span::raw("No estajo selected")])])
        }
    } else {
        Text::from(vec![Line::from(vec![Span::raw("No estajo selected")])])
    };
 
    let block = Block::default()
        .title("Selected Estajo")
        .borders(Borders::ALL);
    let paragraph = Paragraph::new(text)
        .block(block)
        .style(Style::default().fg(Color::White));
    f.render_widget(paragraph, area);
 }
 
 pub fn draw<B: Backend>(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ].as_ref())
        .split(f.size());
 
    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ].as_ref())
        .split(chunks[0]);
 
    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ].as_ref())
        .split(chunks[1]);
 
    draw_population(f, app, top_chunks[0]);
    draw_events_log(f, app, top_chunks[1]);
    draw_needs_status(f, app, bottom_chunks[0]);
    draw_selected_estajo(f, app, bottom_chunks[1]);
 }