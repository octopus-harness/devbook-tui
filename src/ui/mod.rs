pub mod detail;
pub mod modals;
pub mod prompt_list;
pub mod sidebar;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::{App, Pane};

pub fn render(f: &mut Frame, app: &App) {
    let area = f.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(area);

    render_header(f, app, chunks[0]);

    // 3-Pane Body
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(28),
            Constraint::Percentage(38),
            Constraint::Min(0),
        ])
        .split(chunks[1]);

    sidebar::render(f, app, body_chunks[0]);
    prompt_list::render(f, app, body_chunks[1]);
    detail::render(f, app, body_chunks[2]);

    render_footer(f, app, chunks[2]);

    // Modals overlay
    modals::render(f, app);
}

fn render_header(f: &mut Frame, app: &App, area: Rect) {
    let host_badge = format!(" [Host: {}] ", app.config.base_url);
    let stats = &app.stats;
    let stats_text = format!(
        "Prompts: {} (★ {} · Standalone: {}) · Projects: {} · Attachments: {}",
        stats.total_prompts,
        stats.pinned_prompts,
        stats.standalone_prompts,
        stats.total_projects,
        stats.total_attachments
    );

    let mut spans = vec![
        Span::styled(
            " ⚡ DEVBOOK ",
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
        Span::styled(
            host_badge,
            Style::default()
                .fg(Color::LightCyan)
                .add_modifier(Modifier::BOLD),
        ),
    ];

    if app.config.allow_host_switcher {
        spans.push(Span::styled(
            " (H to switch) ",
            Style::default().fg(Color::DarkGray),
        ));
    }

    spans.push(Span::styled(
        format!(" · {}", stats_text),
        Style::default().fg(Color::Gray),
    ));

    spans.push(Span::styled(
        format!(" · [View: {}]", app.view_mode.label()),
        Style::default().fg(Color::LightBlue),
    ));

    if let Some(query) = &app.search_query {
        spans.push(Span::styled(
            format!(" · [Filter: \"{}\"] (Esc to clear)", query),
            Style::default()
                .fg(Color::LightCyan)
                .add_modifier(Modifier::BOLD),
        ));
    }

    let p = Paragraph::new(Line::from(spans)).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::DarkGray)),
    );
    f.render_widget(p, area);
}

fn render_footer(f: &mut Frame, app: &App, area: Rect) {
    let status_span = if let Some((msg, is_error)) = &app.status {
        let color = if *is_error { Color::Red } else { Color::Green };
        Span::styled(
            format!(" ● {} ", msg),
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        )
    } else {
        let focus_hint = match app.focused_pane {
            Pane::Sidebar => {
                "Pane 1 (Scopes & Projects): j/k: nav | Enter: open | n: new proj | e: edit | d: del"
            }
            Pane::Prompts => {
                "Pane 2 (Prompts): j/k: nav | Enter: view | p: pin | n: new prompt | e: edit | c/Y: copy | d: del"
            }
            Pane::Detail => {
                "Pane 3 (Detail): j/k/J/K: scroll/tasks | v: view mode | a: append | E: $EDITOR | c/Y: copy"
            }
        };
        Span::styled(
            format!(" {} ", focus_hint),
            Style::default().fg(Color::DarkGray),
        )
    };

    let key_hints_str = if app.config.allow_host_switcher {
        "1/2/3: panes | v: view | /: search | H: host | ?: help | q: quit "
    } else {
        "1/2/3: panes | v: view | /: search | ?: help | q: quit "
    };

    let key_hints = Span::styled(key_hints_str, Style::default().fg(Color::DarkGray));

    let total_width = area.width as usize;
    let left_len = status_span.content.len();
    let right_len = key_hints.content.len();
    let pad = total_width.saturating_sub(left_len + right_len);

    let line = Line::from(vec![status_span, Span::raw(" ".repeat(pad)), key_hints]);

    f.render_widget(Paragraph::new(line), area);
}
