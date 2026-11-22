use devbook_tui::api::ApiClient;
use devbook_tui::app::{App, ViewMode};
use devbook_tui::checklist::checklist_progress;
use devbook_tui::config::Config;
use devbook_tui::ui;
use ratatui::{backend::TestBackend, Terminal};

#[test]
#[ignore] // Run manually via `cargo test -- --ignored`
fn test_live_instance_and_ui_render() {
    let test_url = std::env::var("DEVBOOK_TEST_URL")
        .or_else(|_| std::env::var("DEVBOOK_REMOTE_URL"))
        .unwrap_or_else(|_| "http://localhost:3000".to_string());

    let config = Config::new(&test_url);
    let client = ApiClient::new(&config);

    if let Ok(stats) = client.get_stats() {
        println!(
            "Connected to Devbook at {}: {} prompts ({} pinned, {} standalone), {} projects, {} attachments",
            test_url,
            stats.total_prompts,
            stats.pinned_prompts,
            stats.standalone_prompts,
            stats.total_projects,
            stats.total_attachments
        );
        if let Ok(prompts) = client.list_prompts(None, None, None, None) {
            for prompt in &prompts {
                if let Ok(detail) = client.get_prompt(prompt.id) {
                    let (done, total) = checklist_progress(detail.body.as_deref().unwrap_or(""));
                    println!(
                        "Checked Prompt #{}: '{}' - {} checklist items ({} completed)",
                        detail.id, detail.title, total, done
                    );
                }
            }
        }

        // Verify full TUI rendering with live data in both Preview and Raw modes
        let mut app = App::new(config);
        let backend = TestBackend::new(120, 36);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal.draw(|f| ui::render(f, &app)).unwrap();
        app.view_mode = ViewMode::Raw;
        terminal.draw(|f| ui::render(f, &app)).unwrap();
    } else {
        panic!("Failed to connect to Devbook at {}", test_url);
    }
}
