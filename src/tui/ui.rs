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
        .split(f.area());

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
    let help_text = if app.is_in_delete_confirmation() {
        Paragraph::new("Y: 削除確定  N: キャンセル  Esc: キャンセル")
            .style(Style::default().fg(Color::Red))
            .block(Block::default().borders(Borders::ALL))
    } else {
        Paragraph::new("↑/↓: 選択  Enter: 開く  d: 削除  q: 終了")
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

    // 削除確認ダイアログ
    if app.is_in_delete_confirmation() {
        if let Some(workspace) = app.get_selected_workspace() {
            draw_delete_confirmation_dialog(f, &workspace.name, &workspace.path);
        }
    }
}

fn draw_delete_confirmation_dialog(f: &mut Frame, workspace_name: &str, workspace_path: &str) {
    // 画面中央にモーダルダイアログを表示
    let area = f.area();
    let popup_width = 60.min(area.width);
    let popup_height = 8.min(area.height);

    let popup_area = ratatui::layout::Rect {
        x: (area.width.saturating_sub(popup_width)) / 2,
        y: (area.height.saturating_sub(popup_height)) / 2,
        width: popup_width,
        height: popup_height,
    };

    // 背景をクリア
    f.render_widget(
        Block::default()
            .style(Style::default().bg(Color::Black))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Red)),
        popup_area,
    );

    // ダイアログ内容
    let dialog_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(1), // タイトル
            Constraint::Length(2), // ワークスペース情報
            Constraint::Length(1), // 確認メッセージ
            Constraint::Length(1), // 操作ガイド
        ])
        .split(popup_area);

    // タイトル
    let title = Paragraph::new("ワークスペースを削除しますか？")
        .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));
    f.render_widget(title, dialog_layout[0]);

    // ワークスペース情報
    let workspace_info = Paragraph::new(format!(
        "名前: {}\nパス: {}",
        workspace_name, workspace_path
    ))
    .style(Style::default().fg(Color::White));
    f.render_widget(workspace_info, dialog_layout[1]);

    // 確認メッセージ
    let warning =
        Paragraph::new("この操作は取り消せません。").style(Style::default().fg(Color::Yellow));
    f.render_widget(warning, dialog_layout[2]);

    // 操作ガイド
    let guide = Paragraph::new("[Y]es  [N]o").style(
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    );
    f.render_widget(guide, dialog_layout[3]);
}
