use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::app::app::App;

// so i will divide it into 2 big boxes
// umm peers and chat

// i like this idea, the ui is nice

pub fn draw(frame: &mut Frame, app: &App) {
    let panels = Layout::default()
    .direction(Direction::Horizontal)
    // the chat is obv bigger
    .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
    .split(frame.area());

    draw_peers(frame, app, panels[0]);
    draw_chat(frame, app, panels[1]);
}

fn draw_peers(frame: &mut Frame, app:&App, area: Rect) {
    // the list of the peers
    // why is this shi so over complicated for no reason??

    // i feel like im just here for shits and giggles, i could never figure this out holy

    let items: Vec<ListItem> = app.peers
        .iter()
        .map(|p| ListItem::new(p.as_str()))
        .collect();

    let list = List::new(items).block(Block::default().borders(Borders::ALL).title(" Peers"));
    frame.render_widget(list, area);
}

fn draw_chat(frame: &mut Frame, app: &App, area: Rect) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)])
        .split(area);

    let height = rows[0].height.saturating_sub(2) as usize;
    let scroll = app.messages.len().saturating_sub(height) as u16;

    let lines: Vec<ratatui::text::Line> = app.messages
        .iter()
        .map(|m| ratatui::text::Line::from(m.as_str()))
        .collect();

    let msg = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title(" Messages"))
        .scroll((scroll, 0));

    frame.render_widget(msg, rows[0]);

    let input = Paragraph::new(app.input.as_str())
        .block(Block::default().borders(Borders::ALL).title(" Input "));

    frame.render_widget(input, rows[1]);
    frame.set_cursor_position((rows[1].x + 1 + app.input.len() as u16, rows[1].y + 1));
}
