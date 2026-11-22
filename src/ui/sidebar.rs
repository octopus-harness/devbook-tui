use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem},
    Frame,
};

use crate::app::{App, Pane};

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let is_focused = app.focused_pane == Pane::Sidebar;
    let border_color = if is_focused {
        Color::Cyan
    } else {
        Color::DarkGray
    };

    let title_style = if is_focused {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };

    let mut items = Vec::new();

    // Virtual Scopes
    let scopes = [
        ("All Prompts", app.stats.total_prompts),
        ("★ Pinned", app.stats.pinned_prompts),
        ("Standalone", app.stats.standalone_prompts),
    ];

    for (i, (label, count)) in scopes.iter().enumerate() {
        let is_selected = app.sidebar_selected == i;
        let prefix = if is_selected { "▶ " } else { "  " };

        let style = if is_selected {
            Style::default()
                .fg(Color::LightCyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        let count_str = format!("({})", count);
        let line = Line::from(vec![
            Span::styled(prefix, style),
            Span::styled(format!("{:<14}", label), style),
            Span::styled(count_str, Style::default().fg(Color::DarkGray)),
        ]);
        items.push(ListItem::new(line));
    }

    // Divider
    items.push(ListItem::new(Line::from(Span::styled(
        " ───────────────────────",
        Style::default().fg(Color::DarkGray),
    ))));

    // Projects Header
    items.push(ListItem::new(Line::from(Span::styled(
        "  PROJECTS",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ))));

    if app.projects.is_empty() {
        items.push(ListItem::new(Line::from(Span::styled(
            "   (no projects)",
            Style::default().fg(Color::DarkGray),
        ))));
    } else {
        for (idx, proj) in app.projects.iter().enumerate() {
            let actual_idx = idx + 3;
            let is_selected = app.sidebar_selected == actual_idx;
            let prefix = if is_selected { "▶ " } else { "  " };

            let style = if is_selected {
                Style::default()
                    .fg(Color::LightCyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            let count_str = format!("({})", proj.prompts_count);
            let name_truncated = if proj.name.chars().count() > 14 {
                let s: String = proj.name.chars().take(13).collect();
                format!("{}…", s)
            } else {
                proj.name.clone()
            };

            let line = Line::from(vec![
                Span::styled(prefix, style),
                Span::styled(format!("{:<14}", name_truncated), style),
                Span::styled(count_str, Style::default().fg(Color::DarkGray)),
            ]);
            items.push(ListItem::new(line));
        }
    }

    // Divider
    items.push(ListItem::new(Line::from(Span::styled(
        " ───────────────────────",
        Style::default().fg(Color::DarkGray),
    ))));

    // Pinned Header
    items.push(ListItem::new(Line::from(Span::styled(
        "  PINNED",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ))));

    if app.pinned_prompts.is_empty() {
        items.push(ListItem::new(Line::from(Span::styled(
            "   (no pinned prompts)",
            Style::default().fg(Color::DarkGray),
        ))));
    } else {
        for (idx, prompt) in app.pinned_prompts.iter().enumerate() {
            let actual_idx = 3 + app.projects.len() + idx;
            let is_selected = app.sidebar_selected == actual_idx;
            let prefix = if is_selected { "▶ " } else { "  " };

            let style = if is_selected {
                Style::default()
                    .fg(Color::LightCyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            let char_count = prompt.title.chars().count();
            let name_truncated = if char_count > 22 {
                let s: String = prompt.title.chars().take(21).collect();
                format!("{}…", s)
            } else {
                prompt.title.clone()
            };

            let line = Line::from(vec![
                Span::styled(prefix, style),
                Span::styled(name_truncated, style),
            ]);
            items.push(ListItem::new(line));
        }
    }

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(border_color))
        .title(Span::styled(" [1] Scopes & Projects ", title_style));

    let list = List::new(items).block(block);
    f.render_widget(list, area);
}
