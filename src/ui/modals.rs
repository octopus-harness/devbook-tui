use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, DeleteTarget, Modal};
use crate::format::format_bytes;

pub fn render(f: &mut Frame, app: &App) {
    let modal = match &app.modal {
        Some(m) => m,
        None => return,
    };

    match modal {
        Modal::HostSwitcher {
            cursor,
            custom_input,
            entering_custom,
        } => render_host_switcher(f, app, *cursor, custom_input, *entering_custom),
        Modal::Search { input } => render_search(f, input),
        Modal::QuickAppendPrompt {
            content_input,
            section_input,
            as_task,
            focus_field,
        } => render_quick_append(f, content_input, section_input, *as_task, *focus_field),
        Modal::ProjectForm {
            id,
            name_input,
            desc_input,
            focus_desc,
        } => render_project_form(f, id.is_some(), name_input, desc_input, *focus_desc),
        Modal::PromptForm {
            id,
            title_input,
            project_choice_idx,
            pinned,
            focus_field,
        } => render_prompt_form(
            f,
            app,
            id.is_some(),
            title_input,
            *project_choice_idx,
            *pinned,
            *focus_field,
        ),
        Modal::ConfirmDelete { target } => render_confirm_delete(f, target),
        Modal::Attachments {
            cursor,
            upload_input,
            dest_input,
        } => render_attachments(
            f,
            app,
            *cursor,
            upload_input.as_deref(),
            dest_input.as_deref(),
        ),
        Modal::Help => render_help(f, app.config.allow_host_switcher),
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn render_host_switcher(
    f: &mut Frame,
    app: &App,
    cursor: usize,
    custom_input: &str,
    entering_custom: bool,
) {
    let area = centered_rect(60, 45, f.area());
    f.render_widget(Clear, area);

    let block = Block::default()
        .title(" Switch Devbook Host URL ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut lines = Vec::new();
    lines.push(Line::from(Span::styled(
        format!("Current Active Host: {}", app.config.base_url),
        Style::default()
            .fg(Color::LightCyan)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "Preset Hosts:",
        Style::default().fg(Color::Gray),
    )));

    for (i, preset) in app.preset_hosts.iter().enumerate() {
        let is_sel = !entering_custom && cursor == i;
        let prefix = if is_sel { " ▶ " } else { "   " };
        let style = if is_sel {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else if *preset == app.config.base_url {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::White)
        };
        let active_mark = if *preset == app.config.base_url {
            " (active)"
        } else {
            ""
        };
        lines.push(Line::from(vec![
            Span::styled(prefix, style),
            Span::styled(preset.as_str(), style),
            Span::styled(active_mark, Style::default().fg(Color::Green)),
        ]));
    }

    lines.push(Line::from(""));
    let custom_sel = entering_custom || cursor == app.preset_hosts.len();
    let custom_prefix = if custom_sel {
        " ▶ Custom URL: "
    } else {
        "   Custom URL: "
    };
    let custom_style = if custom_sel {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };

    lines.push(Line::from(vec![
        Span::styled(custom_prefix, custom_style),
        Span::styled(
            if custom_input.is_empty() && !entering_custom {
                "(type or press c to enter)"
            } else {
                custom_input
            },
            if entering_custom {
                Style::default()
                    .fg(Color::White)
                    .bg(Color::DarkGray)
            } else {
                Style::default().fg(Color::Gray)
            },
        ),
    ]));

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "Keys: j/k: select preset | Enter: switch | c: custom input | Esc: cancel",
        Style::default().fg(Color::DarkGray),
    )));

    f.render_widget(Paragraph::new(lines).wrap(Wrap { trim: false }), inner);
}

fn render_search(f: &mut Frame, input: &str) {
    let area = centered_rect(60, 20, f.area());
    f.render_widget(Clear, area);

    let block = Block::default()
        .title(" Search Prompts ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let lines = vec![
        Line::from(Span::styled(
            "Type search query (matches title or prompt body):",
            Style::default().fg(Color::Gray),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled(" Query: ", Style::default().fg(Color::LightCyan)),
            Span::styled(
                format!("{}_", input),
                Style::default()
                    .fg(Color::White)
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Press Enter to apply filter, Esc to cancel / clear",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    f.render_widget(Paragraph::new(lines), inner);
}

fn render_quick_append(
    f: &mut Frame,
    content_input: &str,
    section_input: &str,
    as_task: bool,
    focus_field: usize,
) {
    let area = centered_rect(65, 45, f.area());
    f.render_widget(Clear, area);

    let block = Block::default()
        .title(" Quick Append to Prompt ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let sec_style = if focus_field == 0 {
        Style::default()
            .fg(Color::White)
            .bg(Color::DarkGray)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };

    let content_style = if focus_field == 1 {
        Style::default()
            .fg(Color::White)
            .bg(Color::DarkGray)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };

    let task_mode_str = if as_task {
        "[x] Append as checklist task (- [ ] ...)"
    } else {
        "[ ] Append as normal Markdown prose"
    };

    let task_style = if as_task {
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };

    let lines = vec![
        Line::from(Span::styled(
            "Optional Section Heading (e.g. 'Instructions' or 'Examples'):",
            Style::default().fg(Color::LightCyan),
        )),
        Line::from(vec![
            Span::styled(" Section: ## ", Style::default().fg(Color::Cyan)),
            Span::styled(format!("{}_", section_input), sec_style),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Content to append:",
            Style::default().fg(Color::LightCyan),
        )),
        Line::from(vec![
            Span::styled(" Text:    ", Style::default().fg(Color::Cyan)),
            Span::styled(format!("{}_", content_input), content_style),
        ]),
        Line::from(""),
        Line::from(Span::styled(task_mode_str, task_style)),
        Line::from(""),
        Line::from(Span::styled(
            "Keys: Tab: switch field | Ctrl+T: toggle task mode | Enter: append | Esc: cancel",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    f.render_widget(Paragraph::new(lines).wrap(Wrap { trim: false }), inner);
}

fn render_project_form(
    f: &mut Frame,
    is_edit: bool,
    name_input: &str,
    desc_input: &str,
    focus_desc: bool,
) {
    let area = centered_rect(55, 35, f.area());
    f.render_widget(Clear, area);

    let title = if is_edit {
        " Edit Project "
    } else {
        " Create New Project "
    };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let name_style = if !focus_desc {
        Style::default()
            .fg(Color::White)
            .bg(Color::DarkGray)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };

    let desc_style = if focus_desc {
        Style::default()
            .fg(Color::White)
            .bg(Color::DarkGray)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };

    let lines = vec![
        Line::from(Span::styled(
            "Project Name (required):",
            Style::default().fg(Color::LightCyan),
        )),
        Line::from(vec![
            Span::styled(" Name: ", Style::default().fg(Color::Cyan)),
            Span::styled(format!("{}_", name_input), name_style),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Description (optional):",
            Style::default().fg(Color::LightCyan),
        )),
        Line::from(vec![
            Span::styled(" Desc: ", Style::default().fg(Color::Cyan)),
            Span::styled(format!("{}_", desc_input), desc_style),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Keys: Tab: switch field | Enter: save | Esc: cancel",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    f.render_widget(Paragraph::new(lines).wrap(Wrap { trim: false }), inner);
}

fn render_prompt_form(
    f: &mut Frame,
    app: &App,
    is_edit: bool,
    title_input: &str,
    project_choice_idx: usize,
    pinned: bool,
    focus_field: usize,
) {
    let area = centered_rect(65, 45, f.area());
    f.render_widget(Clear, area);

    let modal_title = if is_edit {
        " Edit Prompt Metadata "
    } else {
        " Create New Prompt "
    };

    let block = Block::default()
        .title(modal_title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let title_style = if focus_field == 0 {
        Style::default()
            .fg(Color::White)
            .bg(Color::DarkGray)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };

    let proj_label = if project_choice_idx == 0 {
        "(None — Standalone scratchpad)".to_string()
    } else if let Some(p) = app.projects.get(project_choice_idx - 1) {
        format!("{} (ID: {})", p.name, p.id)
    } else {
        "(None)".to_string()
    };

    let proj_style = if focus_field == 1 {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };

    let pin_str = if pinned {
        "[x] Pinned (★)"
    } else {
        "[ ] Not Pinned"
    };

    let pin_style = if focus_field == 2 {
        Style::default()
            .fg(Color::LightCyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };

    let lines = vec![
        Line::from(Span::styled(
            "Prompt Title (required):",
            Style::default().fg(Color::LightCyan),
        )),
        Line::from(vec![
            Span::styled(" Title:   ", Style::default().fg(Color::Cyan)),
            Span::styled(format!("{}_", title_input), title_style),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Assigned Project (Left/Right to choose):",
            Style::default().fg(Color::LightCyan),
        )),
        Line::from(vec![
            Span::styled(" Project: ", Style::default().fg(Color::Cyan)),
            Span::styled(format!("◀ {} ▶", proj_label), proj_style),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(" Pinned:  ", Style::default().fg(Color::Cyan)),
            Span::styled(pin_str, pin_style),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Keys: Tab/Shift-Tab: navigate | h/l: select project | Space: toggle pin | Enter: save | Esc: cancel",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    f.render_widget(Paragraph::new(lines).wrap(Wrap { trim: false }), inner);
}

fn render_confirm_delete(f: &mut Frame, target: &DeleteTarget) {
    let area = centered_rect(50, 25, f.area());
    f.render_widget(Clear, area);

    let (type_name, obj_name) = match target {
        DeleteTarget::Project(p) => ("Project", p.name.as_str()),
        DeleteTarget::Prompt(p) => ("Prompt", p.title.as_str()),
        DeleteTarget::Attachment { filename, .. } => ("Attachment", filename.as_str()),
    };

    let block = Block::default()
        .title(format!(" Confirm Delete {} ", type_name))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Red));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let lines = vec![
        Line::from(Span::styled(
            format!("Are you sure you want to delete this {}?", type_name),
            Style::default().fg(Color::White),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!("  ▶ \"{}\"", obj_name),
            Style::default()
                .fg(Color::Red)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Press 'y' to confirm delete, 'n' or Esc to cancel",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    f.render_widget(Paragraph::new(lines).wrap(Wrap { trim: false }), inner);
}

fn render_attachments(
    f: &mut Frame,
    app: &App,
    cursor: usize,
    upload_input: Option<&str>,
    dest_input: Option<&str>,
) {
    let area = centered_rect(75, 65, f.area());
    f.render_widget(Clear, area);

    let prompt_title = app
        .current_prompt
        .as_ref()
        .map(|p| p.title.as_str())
        .unwrap_or("Prompt");

    let block = Block::default()
        .title(format!(" Attachments Manager — \"{}\" ", prompt_title))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan));

    let inner = block.inner(area);
    f.render_widget(block, area);

    // If entering upload path or download destination
    if let Some(input) = upload_input {
        let input_lines = vec![
            Line::from(Span::styled(
                "Upload File to Prompt Attachments",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "Enter absolute or relative path to file:",
                Style::default().fg(Color::Gray),
            )),
            Line::from(vec![
                Span::styled(" Path: ", Style::default().fg(Color::LightCyan)),
                Span::styled(
                    format!("{}_", input),
                    Style::default()
                        .fg(Color::White)
                        .bg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(""),
            Line::from(Span::styled(
                "Press Enter to upload, Esc to cancel",
                Style::default().fg(Color::DarkGray),
            )),
        ];
        f.render_widget(Paragraph::new(input_lines), inner);
        return;
    }

    if let Some(input) = dest_input {
        let input_lines = vec![
            Line::from(Span::styled(
                "Save Attachment to Local Disk",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "Enter destination file path:",
                Style::default().fg(Color::Gray),
            )),
            Line::from(vec![
                Span::styled(" Dest: ", Style::default().fg(Color::LightCyan)),
                Span::styled(
                    format!("{}_", input),
                    Style::default()
                        .fg(Color::White)
                        .bg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(""),
            Line::from(Span::styled(
                "Press Enter to download, Esc to cancel",
                Style::default().fg(Color::DarkGray),
            )),
        ];
        f.render_widget(Paragraph::new(input_lines), inner);
        return;
    }

    let mut items = Vec::new();
    let attachments = app
        .current_prompt
        .as_ref()
        .map(|p| &p.attachments)
        .cloned()
        .unwrap_or_default();

    if attachments.is_empty() {
        items.push(ListItem::new(Line::from(Span::styled(
            "  (no attachments on this prompt — press 'u' to upload a file)",
            Style::default().fg(Color::DarkGray),
        ))));
    } else {
        for (i, att) in attachments.iter().enumerate() {
            let is_sel = cursor == i;
            let prefix = if is_sel { "▶ " } else { "  " };
            let style = if is_sel {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            let size_str = format_bytes(att.byte_size.unwrap_or(0));
            let line = Line::from(vec![
                Span::styled(prefix, style),
                Span::styled(format!("{:<30} ", att.filename), style),
                Span::styled(
                    format!("{:>10} ", size_str),
                    Style::default().fg(Color::LightCyan),
                ),
                Span::styled(
                    att.content_type.as_deref().unwrap_or("unknown"),
                    Style::default().fg(Color::DarkGray),
                ),
            ]);
            items.push(ListItem::new(line));
        }
    }

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(2)])
        .split(inner);

    f.render_widget(List::new(items), layout[0]);

    let footer = Paragraph::new(Line::from(Span::styled(
        "Keys: j/k: select | u: upload file | s: save to disk | y: copy URL | d: delete | Esc: close",
        Style::default().fg(Color::DarkGray),
    )));
    f.render_widget(footer, layout[1]);
}

fn render_help(f: &mut Frame, allow_switch: bool) {
    let area = centered_rect(76, 78, f.area());
    f.render_widget(Clear, area);

    let block = Block::default()
        .title(" Devbook-TUI Keyboard Shortcuts & Help ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let host_line = if allow_switch {
        Line::from("  H / u          Switch Host URL presets or enter custom URL")
    } else {
        Line::from(
            "  H / u          [Locked by DEVBOOK_REMOTE_URL] (pass --allow-switch to enable)",
        )
    };

    let lines = vec![
        Line::from(Span::styled(
            "Navigation & Focus",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("  1, 2, 3        Direct focus to pane 1 (Sidebar), 2 (Prompts), 3 (Detail)"),
        Line::from("  Tab / BackTab  Cycle focus forward / backward between panes"),
        Line::from("  h / l          Move focus left / right"),
        Line::from("  j / k          Move selection down / up in focused pane"),
        Line::from("  Enter          Enter pane 2 from sidebar, or pane 3 from prompts list"),
        Line::from(""),
        Line::from(Span::styled(
            "Prompt Detail, Markdown & Checklists (Pane 3)",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("  v / m          Toggle between Styled Preview and Raw Markdown (line numbers)"),
        Line::from("  j / k          Scroll prompt (or navigate checklist items when present)"),
        Line::from("  Space / Enter  Toggle active checklist item [ ] ⇄ [x] (if prompt has tasks)"),
        Line::from("  a              Quick-append Markdown section, log entry, or task to prompt"),
        Line::from("  E              Open prompt Markdown body in external $EDITOR (nvim/vim/nano)"),
        Line::from("  J / K          Scroll detail document view down / up by 3 lines"),
        Line::from("  g / G          Jump to top / bottom of prompt document"),
        Line::from("  c / Y          Copy entire Markdown body to system clipboard"),
        Line::from("  y              Yank active task text (or prompt title) to system clipboard"),
        Line::from(""),
        Line::from(Span::styled(
            "Workspace Management & Modals",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        host_line,
        Line::from("  /              Search & filter prompts by title or body in real-time"),
        Line::from("  n              Create New Project (in pane 1) or Prompt (in pane 2/3)"),
        Line::from("  e              Edit metadata of selected Project or Prompt"),
        Line::from("  d              Delete selected Project, Prompt, or Attachment"),
        Line::from("  p              Toggle pinned status of selected Prompt"),
        Line::from("  K / J          Reorder pinned prompts up / down (in Pinned scope)"),
        Line::from("  A              Open attachments manager modal (upload, download, delete)"),
        Line::from("  r              Refresh all data from Devbook server"),
        Line::from("  ?              Open this help dialog"),
        Line::from("  q              Quit application (or Esc to close dialogs)"),
    ];

    f.render_widget(Paragraph::new(lines).wrap(Wrap { trim: false }), inner);
}
