use crate::tui::App;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Content
        ])
        .split(f.size());

    // Header
    let header = Paragraph::new("AI Workspace Manager")
        .style(Style::default().fg(Color::Cyan))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("ワークスペース管理"),
        );
    f.render_widget(header, chunks[0]);

    // Help text
    let help_text = Paragraph::new("↑/↓: 選択  q: 終了")
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL));

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
        let empty_msg = Paragraph::new("ワークスペースが見つかりません。\n\n'gwork start <task-name>' でワークスペースを作成してください。")
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("ワークスペース一覧"));
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
                    Line::from(vec![Span::styled(
                        "  └─ Status: Clean  Files: --  Size: --",
                        Style::default().fg(Color::Green),
                    )]),
                ];

                ListItem::new(content).style(style)
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("ワークスペース一覧 ({} 件)", app.workspaces.len())),
            )
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("→ ");

        let mut list_state = ListState::default();
        list_state.select(Some(app.selected_index));

        f.render_stateful_widget(list, content_layout[1], &mut list_state);
    }
}
