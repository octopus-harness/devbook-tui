use devbook_tui::api::ApiClient;
use devbook_tui::app::{App, DeleteTarget, Modal, Pane, ViewMode};
use devbook_tui::checklist::{
    append_to_prompt, checklist_progress, parse_checklist_items, toggle_checklist_item_by_index,
};
use devbook_tui::config::Config;
use devbook_tui::models::{
    Attachment, Project, Prompt, PromptDetail, Stats,
};
use devbook_tui::ui;
use devbook_tui::ui::detail::format_inline_with_variables;
use ratatui::{backend::TestBackend, style::Style, Terminal};

#[test]
fn test_models_json_serde() {
    let stats_json = r#"{
        "total_prompts": 42,
        "standalone_prompts": 12,
        "pinned_prompts": 5,
        "total_projects": 3,
        "total_attachments": 7
    }"#;
    let stats: Stats = serde_json::from_str(stats_json).expect("Deserialization failed");
    assert_eq!(stats.total_prompts, 42);
    assert_eq!(stats.standalone_prompts, 12);
    assert_eq!(stats.pinned_prompts, 5);

    let prompt_json = r#"{
        "id": 1,
        "project_id": 2,
        "project_name": "Dev Work",
        "title": "System Reviewer",
        "body": "Review the code: {{ code }}",
        "pinned": true,
        "pinned_position": 1,
        "attachments_count": 2,
        "created_at": "2026-11-20T19:21:28Z",
        "updated_at": "2026-11-20T19:21:30Z",
        "attachments": [
            {
                "id": 10,
                "filename": "context.txt",
                "content_type": "text/plain",
                "byte_size": 1024,
                "url": "/rails/blobs/10",
                "created_at": "2026-11-20T19:21:28Z"
            }
        ]
    }"#;
    let detail: PromptDetail = serde_json::from_str(prompt_json).expect("Deserialization failed");
    assert_eq!(detail.title, "System Reviewer");
    assert_eq!(detail.attachments.len(), 1);
    assert_eq!(detail.attachments[0].filename, "context.txt");

    let prompt: Prompt = detail.into();
    assert_eq!(prompt.id, 1);
    assert!(prompt.pinned);
}

#[test]
fn test_template_variable_highlighting() {
    let text = "Hello {{ user }}, please review {{ pull request }} today!";
    let spans = format_inline_with_variables(text, Style::default());
    assert_eq!(spans.len(), 5);
    assert_eq!(spans[0].content, "Hello ");
    assert_eq!(spans[1].content, "{{ user }}");
    assert_eq!(spans[2].content, ", please review ");
    assert_eq!(spans[3].content, "{{ pull request }}");
    assert_eq!(spans[4].content, " today!");
}

#[test]
fn test_checklist_and_append() {
    let md = "# System Prompt\n- [ ] Task 1\n- [x] Task 2\n";
    let (done, total) = checklist_progress(md);
    assert_eq!(done, 1);
    assert_eq!(total, 2);

    let (toggled, _) = toggle_checklist_item_by_index(md, 0).expect("Toggle failed");
    let (done_after, _) = checklist_progress(&toggled);
    assert_eq!(done_after, 2);

    let appended = append_to_prompt(&toggled, "New Task", Some("Section"), true);
    assert!(appended.contains("## Section"));
    assert!(appended.contains("- [ ] New Task"));
}

#[test]
fn test_ui_rendering_with_test_backend() {
    let config = Config::new("http://localhost:3000");
    let mut app = App {
        config: config.clone(),
        client: ApiClient::new(&config),
        should_quit: false,
        pending_editor: None,
        status: Some(("Welcome to Devbook-TUI".to_string(), false)),
        focused_pane: Pane::Prompts,
        view_mode: ViewMode::Preview,
        modal: None,
        stats: Stats {
            total_prompts: 3,
            standalone_prompts: 2,
            pinned_prompts: 1,
            total_projects: 1,
            total_attachments: 1,
        },
        preset_hosts: vec!["http://localhost:3000".to_string()],
        projects: vec![Project {
            id: 1,
            name: "Coding".to_string(),
            description: Some("Development prompts".to_string()),
            prompts_count: 1,
            created_at: None,
            updated_at: None,
        }],
        pinned_prompts: vec![Prompt {
            id: 10,
            project_id: None,
            project_name: None,
            title: "Pinned Prompt".to_string(),
            body: Some("Pinned body".to_string()),
            pinned: true,
            pinned_position: Some(1),
            attachments_count: 0,
            created_at: None,
            updated_at: None,
        }],
        prompts: vec![
            Prompt {
                id: 10,
                project_id: None,
                project_name: None,
                title: "Pinned Prompt".to_string(),
                body: Some("Pinned body".to_string()),
                pinned: true,
                pinned_position: Some(1),
                attachments_count: 0,
                created_at: None,
                updated_at: None,
            },
            Prompt {
                id: 11,
                project_id: Some(1),
                project_name: Some("Coding".to_string()),
                title: "Code Reviewer".to_string(),
                body: Some("Review {{ diff }} with instructions:\n- [ ] check types\n- [x] check syntax".to_string()),
                pinned: false,
                pinned_position: None,
                attachments_count: 1,
                created_at: Some("2026-11-20T19:21:28Z".to_string()),
                updated_at: Some("2026-11-20T19:21:28Z".to_string()),
            },
        ],
        current_prompt: Some(PromptDetail {
            id: 11,
            project_id: Some(1),
            project_name: Some("Coding".to_string()),
            title: "Code Reviewer".to_string(),
            body: Some("Review {{ diff }} with instructions:\n- [ ] check types\n- [x] check syntax".to_string()),
            pinned: false,
            pinned_position: None,
            attachments_count: 1,
            created_at: Some("2026-11-20T19:21:28Z".to_string()),
            updated_at: Some("2026-11-20T19:21:28Z".to_string()),
            rendered_html: None,
            attachments: vec![Attachment {
                id: 1,
                filename: "patch.diff".to_string(),
                content_type: Some("text/plain".to_string()),
                byte_size: Some(2048),
                url: "/rails/blobs/1".to_string(),
                created_at: None,
            }],
        }),
        checklist_items: parse_checklist_items("Review {{ diff }} with instructions:\n- [ ] check types\n- [x] check syntax"),
        sidebar_selected: 0,
        prompts_selected: 1,
        detail_selected_checklist: Some(0),
        detail_scroll: 0,
        search_query: None,
        clipboard: None,
    };

    let backend = TestBackend::new(120, 36);
    let mut terminal = Terminal::new(backend).unwrap();

    // 1. Render in Preview mode
    terminal.draw(|f| ui::render(f, &app)).unwrap();

    // 2. Render in Raw mode
    app.view_mode = ViewMode::Raw;
    terminal.draw(|f| ui::render(f, &app)).unwrap();

    // 3. Render Help modal
    app.modal = Some(Modal::Help);
    terminal.draw(|f| ui::render(f, &app)).unwrap();

    // 4. Render Search modal
    app.modal = Some(Modal::Search { input: "review".to_string() });
    terminal.draw(|f| ui::render(f, &app)).unwrap();

    // 5. Render QuickAppend modal
    app.modal = Some(Modal::QuickAppendPrompt {
        content_input: "new instruction".to_string(),
        section_input: "Guidelines".to_string(),
        as_task: false,
        focus_field: 0,
    });
    terminal.draw(|f| ui::render(f, &app)).unwrap();

    // 6. Render PromptForm modal
    app.modal = Some(Modal::PromptForm {
        id: None,
        title_input: "New Prompt".to_string(),
        project_choice_idx: 0,
        pinned: false,
        focus_field: 0,
    });
    terminal.draw(|f| ui::render(f, &app)).unwrap();

    // 7. Render ConfirmDelete modal
    app.modal = Some(Modal::ConfirmDelete {
        target: DeleteTarget::Prompt(app.prompts[0].clone()),
    });
    terminal.draw(|f| ui::render(f, &app)).unwrap();

    // 8. Render Attachments modal
    app.modal = Some(Modal::Attachments {
        cursor: 0,
        upload_input: None,
        dest_input: None,
    });
    terminal.draw(|f| ui::render(f, &app)).unwrap();
}
