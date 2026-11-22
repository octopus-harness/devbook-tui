# devbook-tui

<p align="center">
  <strong>A fast, keyboard-driven terminal user interface (TUI) for the <a href="https://github.com/octopus-harness/devbook">Devbook</a> prompt engineering workbench and developer workspace.</strong>
</p>

<p align="center">
  <a href="https://www.rust-lang.org"><img src="https://img.shields.io/badge/Rust-2021_Edition-dea584?style=flat-square&logo=rust&logoColor=white" alt="Rust 2021"></a>
  <a href="https://ratatui.rs"><img src="https://img.shields.io/badge/Ratatui-0.28-111111?style=flat-square" alt="Ratatui"></a>
  <a href="https://github.com/crossterm-rs/crossterm"><img src="https://img.shields.io/badge/Crossterm-0.28-blue?style=flat-square" alt="Crossterm"></a>
  <img src="https://img.shields.io/badge/Theme-Cyan_&_Electric_Blue-06b6d4?style=flat-square" alt="Devbook Cyan Theme">
  <a href="https://github.com/octopus-harness/devbook"><img src="https://img.shields.io/badge/Devbook-Rails_8-D30001?style=flat-square&logo=rubyonrails&logoColor=white" alt="Devbook Engine"></a>
  <a href="https://github.com/leouofa/devbook-mcp"><img src="https://img.shields.io/badge/MCP_Server-devbook--mcp-6366f1?style=flat-square&logo=github&logoColor=white" alt="Devbook MCP Server"></a>
  <a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/License-MIT-blue?style=flat-square" alt="License: MIT"></a>
</p>

---

A fast, keyboard-driven terminal user interface (TUI) for the [Devbook](https://github.com/octopus-harness/devbook) prompt engineering workbench and developer workspace, built with [Ratatui](https://ratatui.rs) and [Crossterm](https://github.com/crossterm-rs/crossterm).

## Features

- **Host URL Resolution & Host Switcher**:
  - **Environment default**: When `DEVBOOK_REMOTE_URL` (or `DEVBOOK_URL`) is set in the shell (e.g. `export DEVBOOK_REMOTE_URL="http://100.96.219.81:3008"`), running `cargo run` or `devbook-tui` connects directly to that URL.
  - **Fallback**: If `DEVBOOK_REMOTE_URL` is not set, defaults to `http://localhost:3000`.
  - **CLI overrides**: `--url <URL>`, `--host <URL>`, `--remote` (`http://100.96.219.81:3008`), and `--local` (`http://localhost:3000`) always take precedence over the environment.
  - **Locking behavior**:
    - If `DEVBOOK_REMOTE_URL` is set, host switching is locked by default so the TUI stays pinned to your configured instance.
    - Pass `--allow-switch` to re-enable the Host Switcher dialog while using `DEVBOOK_REMOTE_URL`.
    - If `DEVBOOK_REMOTE_URL` is not set, Host Switcher (`H` or `u`) is enabled automatically.
- **3-Pane Master–Detail Layout (Devbook Cyan & Electric Blue Theme)**:
  - **Header**: Live host URL badge, workspace statistics (Prompts, Pinned, Standalone, Projects, Attachments), active Markdown view mode, and search filter badge.
  - **Pane 1 `[1] Scopes & Projects`**: All Prompts, ★ Pinned, Standalone scratchpads, and all Projects with prompt counts.
  - **Pane 2 `[2] Prompts`**: Filtered list showing pin markers (`★`), prompt titles, optional checklist progress badges (`[done/total]`), attachment counters (`📎`), project badges, and updated dates.
  - **Pane 3 `[3] Prompt Detail [Preview | Raw]`**: Prompt metadata header (project, char/word/line counts, updated timestamp, attachment counters) and rich Markdown body.
- **Template Variable Highlighting & Smart Checklists**:
  - Highlight prompt template placeholders (`{{ variable_name }}`) with distinct styling in preview mode.
  - `v` / `m` toggles between **Styled Preview Mode** (GFM headings, fenced code blocks, blockquotes, tables, horizontal rules, bullet lists, word wrap) and **Raw Markdown Mode** (with line numbers).
  - If a prompt contains `- [ ]` / `- [x]` checklist items, `j` / `k` in Preview mode navigates directly between tasks and `Space` / `Enter` toggles completion state back to the server.
  - `J` / `K`, `PageDown` / `PageUp`, and `g` / `G` scroll the full document smoothly.
- **Full CRUD, Quick Append & External `$EDITOR`**:
  - `a`: **Quick Append to Prompt** modal — append instructions, examples, or checklist items (`- [ ]`) under an optional `## Section` header without leaving the TUI.
  - `n` / `e`: Create or edit Projects (in Pane 1) or Prompts (in Pane 2/3).
  - `d`: Delete Project (preserving prompts as standalone), Prompt, or Attachment with confirmation dialog.
  - `p`: Toggle pin status on selected Prompt.
  - `K` / `J`: Reorder pinned Prompts when viewing the Pinned scope.
  - `E`: Open full Markdown prompt body in external `$EDITOR` (e.g. `nvim`, `vim`, `nano`). Suspends TUI cleanly and saves back to Devbook on exit.
- **Attachments & Zero-Latency Clipboard Copy**:
  - `A`: Open Context Attachment Manager modal to view attachments, upload new files by path (`u`), download files to disk (`s`), copy attachment URLs (`y`), or purge files (`d`).
  - `c` / `Y`: Copy entire Markdown body to system clipboard (`arboard`).
  - `y`: Yank active checklist item text (or prompt title) to system clipboard.
- **Live Search**:
  - `/`: Real-time full-text search across prompt titles and bodies. Press `Esc` to clear.

## Installation & Running

> [!NOTE]
> `devbook-tui` connects to a running [Devbook](https://github.com/octopus-harness/devbook) backend instance (defaults to `DEVBOOK_REMOTE_URL` or `http://localhost:3000`).

### Build from Source

```bash
# Build
cargo build --release

# Run (defaults to DEVBOOK_REMOTE_URL if set, otherwise http://localhost:3000)
cargo run

# Force local host even if DEVBOOK_REMOTE_URL is set
cargo run -- --local

# Specify any target host explicitly
cargo run -- --url http://100.96.219.81:3008

# Allow in-app Host Switcher even when DEVBOOK_REMOTE_URL is set
cargo run -- --allow-switch
```

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `1`, `2`, `3` | Direct focus to Pane 1 (Sidebar), 2 (Prompts), 3 (Detail) |
| `Tab` / `BackTab` | Cycle focus forward / backward between panes |
| `h` / `l` | Move focus left / right |
| `j` / `k` | Move selection down / up in focused pane |
| `Enter` | Enter pane 2 from sidebar, or pane 3 from prompts list |
| `v` / `m` | Toggle between **Preview** (styled GFM with `{{ variable }}` highlights) and **Raw** (line-numbered) Markdown modes |
| `Space` / `Enter` | Toggle active checklist item `[ ]` ⇄ `[x]` (in Pane 3 when tasks exist) |
| `a` | Quick-append Markdown content or task (`- [ ]`) to current prompt |
| `E` | Open prompt Markdown body in `$EDITOR` |
| `J` / `K` | Scroll detail document down / up |
| `g` / `G` | Jump to top / bottom of detail document |
| `c` / `Y` | Copy full prompt Markdown body to system clipboard |
| `y` | Yank active task text or prompt title to system clipboard |
| `H` / `u` | Open Host Switcher dialog (when enabled or via `--allow-switch`) |
| `/` | Live search & filter prompts (`Esc` to clear) |
| `n` | New Project (in pane 1) or Prompt (in pane 2/3) |
| `e` | Edit Project or Prompt metadata |
| `d` | Delete with confirmation |
| `p` | Toggle pin on selected Prompt |
| `K` / `J` | Reorder pinned prompts up / down (in Pinned scope) |
| `A` | Manage prompt attachments |
| `r` | Refresh data from server |
| `?` | Show help modal |
| `q` | Quit application |
