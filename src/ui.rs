use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
    Frame,
};

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(5),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(f.area());

    // PR Table
    let header = Row::new(vec![
        Cell::from("Repo"),
        Cell::from("#"),
        Cell::from("Title"),
        Cell::from("Author"),
        Cell::from("Updated"),
    ])
    .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

    let rows: Vec<Row> = app
        .prs
        .iter()
        .map(|pr| {
            Row::new(vec![
                Cell::from(pr.repo().to_string()),
                Cell::from(format!("#{}", pr.number)),
                Cell::from(pr.title.clone()),
                Cell::from(pr.author().to_string()),
                Cell::from(pr.updated_short().to_string()),
            ])
        })
        .collect();

    let widths = [
        Constraint::Percentage(20),
        Constraint::Length(7),
        Constraint::Percentage(40),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Review Requests "),
        )
        .row_highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    let mut state = TableState::default();
    if !app.prs.is_empty() {
        state.select(Some(app.selected));
    }
    f.render_stateful_widget(table, chunks[0], &mut state);

    // Status bar
    let status = if let Some(ref err) = app.error {
        Line::from(vec![
            Span::styled("ERROR: ", Style::default().fg(Color::Red)),
            Span::raw(err.as_str()),
        ])
    } else {
        Line::from(vec![
            Span::styled(
                format!(" {} PRs", app.prs.len()),
                Style::default().fg(Color::Green),
            ),
            Span::raw(format!(" | Last updated: {}", app.last_updated)),
        ])
    };
    f.render_widget(Paragraph::new(status), chunks[1]);

    // Keybinds help
    let help = Line::from(vec![
        Span::styled(" q", Style::default().fg(Color::Cyan)),
        Span::raw(":quit "),
        Span::styled("j/k", Style::default().fg(Color::Cyan)),
        Span::raw(":move "),
        Span::styled("Enter", Style::default().fg(Color::Cyan)),
        Span::raw(":open "),
        Span::styled("r", Style::default().fg(Color::Cyan)),
        Span::raw(":refresh"),
    ]);
    f.render_widget(Paragraph::new(help), chunks[2]);
}
