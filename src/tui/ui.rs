use crate::tui::App;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

pub fn draw(f: &mut Frame, app: &App, workspace_manager: &crate::workspace::WorkspaceManager) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Content
        ])
        .split(f.area());

    // Header
    let header = Paragraph::new("AI Workspace Manager")
        .style(Style::default().fg(Color::Cyan))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Workspace Management"),
        );
    f.render_widget(header, chunks[0]);

    // Help text
    let help_text = if app.is_in_delete_confirmation() {
        Paragraph::new("Y: Confirm deletion  N: Cancel  Esc: Cancel")
            .style(Style::default().fg(Color::Red))
            .block(Block::default().borders(Borders::ALL))
    } else if app.is_in_details_view() {
        Paragraph::new("Press any key to close")
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::ALL))
    } else {
        Paragraph::new("↑/↓: Select  Enter: Open  d: Delete  i: Details  q: Quit")
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL))
    };

    let content_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Help
            Constraint::Min(0),    // List
        ])
        .split(chunks[1]);

    f.render_widget(help_text, content_layout[0]);

    // Workspace list
    if app.workspaces.is_empty() {
        let empty_msg = Paragraph::new(
            "No workspaces found.\n\nCreate a workspace with 'gwork start <task-name>'.",
        )
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Workspace List"),
        );
        f.render_widget(empty_msg, content_layout[1]);
    } else {
        let items: Vec<ListItem> = app
            .workspaces
            .iter()
            .enumerate()
            .map(|(i, workspace)| {
                let style = if i == app.selected_index {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };

                let content = vec![
                    Line::from(vec![Span::styled(format!("● {}", workspace.branch), style)]),
                    Line::from(vec![Span::styled(
                        format!("  └─ {}", workspace.path),
                        Style::default().fg(Color::Gray),
                    )]),
                ];

                ListItem::new(content).style(style)
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("Workspace List ({} items)", app.workspaces.len())),
            )
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("→ ");

        let mut list_state = ListState::default();
        list_state.select(Some(app.selected_index));

        f.render_stateful_widget(list, content_layout[1], &mut list_state);
    }

    // Details dialog
    if app.is_in_details_view() {
        if let Some(workspace) = app.get_selected_workspace() {
            draw_workspace_details_dialog(f, workspace, workspace_manager);
        }
    }

    // Delete confirmation dialog
    if app.is_in_delete_confirmation() {
        if let Some(workspace) = app.get_selected_workspace() {
            draw_delete_confirmation_dialog(f, &workspace.name, &workspace.path);
        }
    }
}

fn draw_delete_confirmation_dialog(f: &mut Frame, workspace_name: &str, workspace_path: &str) {
    // Display modal dialog in the center of the screen
    let area = f.area();
    let popup_width = 60.min(area.width);
    let popup_height = 8.min(area.height);

    let popup_area = ratatui::layout::Rect {
        x: (area.width.saturating_sub(popup_width)) / 2,
        y: (area.height.saturating_sub(popup_height)) / 2,
        width: popup_width,
        height: popup_height,
    };

    // Clear background
    f.render_widget(
        Block::default()
            .style(Style::default().bg(Color::Black))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Red)),
        popup_area,
    );

    // Dialog content
    let dialog_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(1), // Title
            Constraint::Length(2), // Workspace information
            Constraint::Length(1), // Confirmation message
            Constraint::Length(1), // Operation guide
        ])
        .split(popup_area);

    // Title
    let title = Paragraph::new("Delete workspace?")
        .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));
    f.render_widget(title, dialog_layout[0]);

    // Workspace information
    let workspace_info = Paragraph::new(format!("Name: {workspace_name}\nPath: {workspace_path}"))
        .style(Style::default().fg(Color::White));
    f.render_widget(workspace_info, dialog_layout[1]);

    // Confirmation message
    let warning = Paragraph::new("This operation cannot be undone.")
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(warning, dialog_layout[2]);

    // Operation guide
    let guide = Paragraph::new("[Y]es  [N]o").style(
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    );
    f.render_widget(guide, dialog_layout[3]);
}

fn draw_workspace_details_dialog(
    f: &mut Frame,
    workspace: &crate::workspace::WorkspaceInfo,
    workspace_manager: &crate::workspace::WorkspaceManager,
) {
    // Display modal dialog in the center of the screen
    let area = f.area();
    let popup_width = 80.min(area.width);
    let popup_height = 16.min(area.height);

    let popup_area = ratatui::layout::Rect {
        x: (area.width.saturating_sub(popup_width)) / 2,
        y: (area.height.saturating_sub(popup_height)) / 2,
        width: popup_width,
        height: popup_height,
    };

    // Background block
    f.render_widget(
        Block::default()
            .style(Style::default().bg(Color::Black))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .title("Workspace Details"),
        popup_area,
    );

    // Dialog content layout
    let dialog_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(2), // Basic information
            Constraint::Length(2), // Date/time information
            Constraint::Length(2), // Status information
            Constraint::Length(4), // Recent commits
            Constraint::Length(1), // Operation guide
        ])
        .split(popup_area);

    // Basic information
    let basic_info = Paragraph::new(format!(
        "Branch: {}\nPath: {}",
        workspace.branch, workspace.path
    ))
    .style(Style::default().fg(Color::White));
    f.render_widget(basic_info, dialog_layout[0]);

    // Get detailed information
    match workspace_manager.get_workspace_details(workspace) {
        Ok(details) => {
            // Date/time information
            let time_info = Paragraph::new(format!(
                "Created: {}\nLast Modified: {}",
                details.created, details.last_modified
            ))
            .style(Style::default().fg(Color::Gray));
            f.render_widget(time_info, dialog_layout[1]);

            // Status information
            let status_info = Paragraph::new(format!(
                "Status: {}\nFiles: {}  Size: {}",
                details.status, details.files_info, details.size
            ))
            .style(Style::default().fg(Color::Green));
            f.render_widget(status_info, dialog_layout[2]);

            // Recent commit history
            let commits_text = if details.recent_commits.is_empty() {
                "Recent Commits:\nNone".to_string()
            } else {
                format!("Recent Commits:\n{}", details.recent_commits.join("\n"))
            };
            let commit_info =
                Paragraph::new(commits_text).style(Style::default().fg(Color::Yellow));
            f.render_widget(commit_info, dialog_layout[3]);
        }
        Err(_) => {
            // Display alternative text if error occurs
            let time_info = Paragraph::new("Created: Error\nLast Modified: Error")
                .style(Style::default().fg(Color::Red));
            f.render_widget(time_info, dialog_layout[1]);

            let status_info = Paragraph::new("Status: Error\nFiles: --  Size: --")
                .style(Style::default().fg(Color::Red));
            f.render_widget(status_info, dialog_layout[2]);

            let commit_info =
                Paragraph::new("Recent Commits:\nError").style(Style::default().fg(Color::Red));
            f.render_widget(commit_info, dialog_layout[3]);
        }
    }

    // Operation guide
    let guide = Paragraph::new("Press any key to close").style(
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    );
    f.render_widget(guide, dialog_layout[4]);
}
