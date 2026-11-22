#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChecklistItem {
    pub index: usize,       // 0-based index among checklist items
    pub line_number: usize, // 1-based line index in source markdown
    pub completed: bool,
    pub text: String,
    pub indent: String,
    pub raw: String,
}

/// Matches lines like: `   - [ ] task text` or `* [x] task text`
fn parse_checklist_line(line: &str) -> Option<(String, bool, String)> {
    let trimmed_start = line.trim_start();
    let indent = &line[..(line.len() - trimmed_start.len())];

    let rest = trimmed_start
        .strip_prefix("- ")
        .or_else(|| trimmed_start.strip_prefix("* "))?;

    if !rest.starts_with('[') || rest.len() < 3 {
        return None;
    }

    let mark = rest.as_bytes()[1];
    let completed = match mark {
        b'x' | b'X' => true,
        b' ' => false,
        _ => return None,
    };

    if rest.as_bytes()[2] != b']' {
        return None;
    }

    let text_part = &rest[3..];
    let task_text = if let Some(stripped) = text_part.strip_prefix(' ') {
        stripped
    } else {
        text_part
    };

    Some((indent.to_string(), completed, task_text.to_string()))
}

pub fn parse_checklist_items(markdown: &str) -> Vec<ChecklistItem> {
    let mut items = Vec::new();
    let mut index = 0;

    for (line_idx, line) in markdown.lines().enumerate() {
        if let Some((indent, completed, text)) = parse_checklist_line(line) {
            items.push(ChecklistItem {
                index,
                line_number: line_idx + 1,
                completed,
                text,
                indent,
                raw: line.to_string(),
            });
            index += 1;
        }
    }

    items
}

pub fn checklist_progress(markdown: &str) -> (usize, usize) {
    let items = parse_checklist_items(markdown);
    let total = items.len();
    let completed = items.iter().filter(|i| i.completed).count();
    (completed, total)
}

pub fn toggle_checklist_item_by_index(
    markdown: &str,
    target_idx: usize,
) -> Option<(String, ChecklistItem)> {
    let items = parse_checklist_items(markdown);
    let target = items.iter().find(|i| i.index == target_idx)?;

    let mut lines: Vec<String> = markdown.lines().map(|s| s.to_string()).collect();
    let line_idx = target.line_number - 1;
    if line_idx >= lines.len() {
        return None;
    }

    let current_line = &lines[line_idx];
    let new_completed = !target.completed;
    let new_mark = if new_completed { "x" } else { " " };

    // Replace the checkbox mark while preserving bullet style and indent
    let trimmed_start = current_line.trim_start();
    let indent = &current_line[..(current_line.len() - trimmed_start.len())];
    let bullet = if trimmed_start.starts_with("- ") {
        "- "
    } else {
        "* "
    };
    let rest = &trimmed_start[bullet.len()..]; // should start with [ ] or [x]
    let replaced_line = if rest.len() >= 3 && rest.starts_with('[') && rest.as_bytes()[2] == b']' {
        let text_after = &rest[3..];
        format!("{}{}[{}]{}", indent, bullet, new_mark, text_after)
    } else {
        return None;
    };

    lines[line_idx] = replaced_line.clone();

    let updated_item = ChecklistItem {
        index: target_idx,
        line_number: target.line_number,
        completed: new_completed,
        text: target.text.clone(),
        indent: target.indent.clone(),
        raw: replaced_line,
    };

    let mut result = lines.join("\n");
    if markdown.ends_with('\n') {
        result.push('\n');
    }

    Some((result, updated_item))
}

pub fn normalize_task_line(task_text: &str) -> String {
    let trimmed = task_text.trim();
    if trimmed.starts_with("- [ ]")
        || trimmed.starts_with("- [x]")
        || trimmed.starts_with("- [X]")
    {
        return trimmed.to_string();
    }
    if trimmed.starts_with("[ ]") || trimmed.starts_with("[x]") || trimmed.starts_with("[X]") {
        return format!("- {}", trimmed);
    }
    format!("- [ ] {}", trimmed)
}

/// Appends arbitrary markdown content (or a checklist task if `as_task` is true)
/// to a prompt body, optionally under a matching or newly created `## Section` header.
pub fn append_to_prompt(
    current_markdown: &str,
    content: &str,
    section_header: Option<&str>,
    as_task: bool,
) -> String {
    if content.trim().is_empty() {
        return current_markdown.to_string();
    }

    let formatted_content = if as_task {
        normalize_task_line(content)
    } else {
        content.trim().to_string()
    };

    let mut lines: Vec<String> = current_markdown.lines().map(|s| s.to_string()).collect();

    if let Some(header) = section_header.filter(|h| !h.trim().is_empty()) {
        let header_clean = header.trim().to_lowercase();
        let header_with_hashes = if header_clean.starts_with('#') {
            header_clean.clone()
        } else {
            format!("## {}", header_clean)
        };

        let header_idx = lines.iter().position(|l| {
            let cl = l.trim().to_lowercase();
            cl == header_clean
                || cl == header_with_hashes
                || cl == format!("# {}", header_clean)
                || cl == format!("### {}", header_clean)
        });

        if let Some(idx) = header_idx {
            // Find insertion point before next heading
            let mut insert_idx = lines.len();
            for (i, line) in lines.iter().enumerate().skip(idx + 1) {
                let trimmed = line.trim();
                if trimmed.starts_with('#') {
                    insert_idx = i;
                    break;
                }
            }

            while insert_idx > idx + 1 && lines[insert_idx - 1].trim().is_empty() {
                insert_idx -= 1;
            }

            for (offset, line) in formatted_content.lines().enumerate() {
                lines.insert(insert_idx + offset, line.to_string());
            }

            let mut out = lines.join("\n");
            if current_markdown.ends_with('\n') {
                out.push('\n');
            }
            return out;
        }

        // Header not found: append header and content at end
        let formatted_header = if header.trim().starts_with('#') {
            header.trim().to_string()
        } else {
            format!("## {}", header.trim())
        };

        let base = current_markdown.trim_end();
        if base.is_empty() {
            return format!("{}\n{}\n", formatted_header, formatted_content);
        } else {
            return format!("{}\n\n{}\n{}\n", base, formatted_header, formatted_content);
        }
    }

    let base = current_markdown.trim_end();
    if base.is_empty() {
        format!("{}\n", formatted_content)
    } else if as_task {
        format!("{}\n{}\n", base, formatted_content)
    } else {
        format!("{}\n\n{}\n", base, formatted_content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_progress() {
        let md = r#"# Architecture Notes
- [ ] Review prompt template
- [x] Test model parameters
* [ ] Verify system prompt
Some regular markdown text
  - [X] Nested item
"#;
        let items = parse_checklist_items(md);
        assert_eq!(items.len(), 4);
        assert_eq!(items[0].text, "Review prompt template");
        assert!(!items[0].completed);
        assert_eq!(items[1].text, "Test model parameters");
        assert!(items[1].completed);
        assert_eq!(items[2].text, "Verify system prompt");
        assert_eq!(items[3].indent, "  ");
        assert!(items[3].completed);

        let (done, total) = checklist_progress(md);
        assert_eq!(done, 2);
        assert_eq!(total, 4);
    }

    #[test]
    fn test_toggle_item() {
        let md = "- [ ] Item A\n- [x] Item B\n";
        let (updated, item) = toggle_checklist_item_by_index(md, 0).unwrap();
        assert!(item.completed);
        assert_eq!(updated, "- [x] Item A\n- [x] Item B\n");

        let (toggled_back, item2) = toggle_checklist_item_by_index(&updated, 0).unwrap();
        assert!(!item2.completed);
        assert_eq!(toggled_back, "- [ ] Item A\n- [x] Item B\n");
    }

    #[test]
    fn test_append_prose_and_task() {
        let base = "# Devbook Prompt\n\nInitial instructions.\n";
        let res = append_to_prompt(base, "Add error handling instructions.", Some("Guidelines"), false);
        assert_eq!(
            res,
            "# Devbook Prompt\n\nInitial instructions.\n\n## Guidelines\nAdd error handling instructions.\n"
        );

        let res_task = append_to_prompt(&res, "Check model token limit", Some("Checklist"), true);
        assert!(res_task.contains("## Checklist\n- [ ] Check model token limit\n"));
    }
}
