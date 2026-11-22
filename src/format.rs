pub fn format_date_short(iso: &str) -> String {
    // Expected format: "2026-11-20T19:21:28Z" or "2026-11-20T19:21:28.123Z"
    if iso.len() >= 16 {
        // e.g. "11-20 19:21"
        let month_day = &iso[5..10];
        let hour_min = &iso[11..16];
        format!("{} {}", month_day, hour_min)
    } else if iso.len() >= 10 {
        iso[..10].to_string()
    } else {
        iso.to_string()
    }
}

pub fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

pub fn word_and_line_count(text: &str) -> (usize, usize, usize) {
    let chars = text.chars().count();
    let words = text.split_whitespace().count();
    let lines = if text.is_empty() { 0 } else { text.lines().count() };
    (chars, words, lines)
}

pub fn truncate_str(s: &str, max_width: usize) -> String {
    if s.chars().count() <= max_width {
        s.to_string()
    } else {
        let mut truncated: String = s.chars().take(max_width.saturating_sub(1)).collect();
        truncated.push('…');
        truncated
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_formatting() {
        assert_eq!(format_date_short("2026-11-20T19:21:28Z"), "11-20 19:21");
        assert_eq!(format_date_short("2026-11-20"), "2026-11-20");
        assert_eq!(format_date_short("short"), "short");
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(500), "500 B");
        assert_eq!(format_bytes(1024), "1.0 KB");
        assert_eq!(format_bytes(1536), "1.5 KB");
        assert_eq!(format_bytes(1024 * 1024), "1.0 MB");
    }

    #[test]
    fn test_word_and_line_count() {
        let text = "Hello world\nThis is a prompt\nWith three lines";
        let (chars, words, lines) = word_and_line_count(text);
        assert_eq!(lines, 3);
        assert_eq!(words, 9);
        assert_eq!(chars, text.chars().count());

        let (c, w, l) = word_and_line_count("");
        assert_eq!((c, w, l), (0, 0, 0));
    }
}
