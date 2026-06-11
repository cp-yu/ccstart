mod filter;

use crossterm::{
    cursor, event::{self, Event, KeyCode, KeyModifiers}, execute, queue,
    style::{Color, Print, ResetColor, SetForegroundColor}, terminal::{self, ClearType}
};
use std::io::{self, Write};

enum Mode {
    Normal,
    Filter,
}

pub fn select(items: &[String], title: &str) -> io::Result<Option<String>> {
    if items.is_empty() {
        return Ok(None);
    }

    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, cursor::Hide)?;

    let result = run_selector(&mut stdout, items, title);

    execute!(stdout, cursor::Show)?;
    terminal::disable_raw_mode()?;

    result
}

fn run_selector(stdout: &mut impl Write, items: &[String], title: &str) -> io::Result<Option<String>> {
    let mut mode = Mode::Normal;
    let mut cursor_pos = 0;
    let mut filter_input = String::new();
    let mut filter_cursor = 0;
    let mut kill_slot = String::new();
    let mut pending_g = false;
    let mut scroll_offset = 0usize;

    let (_, term_rows) = terminal::size()?;
    // 2 header lines (title + hint) + 1 for scroll indicator
    let header_rows = 3u16;
    let indicator_rows = 1u16;
    let view_size = ((term_rows.saturating_sub(header_rows + indicator_rows)) as usize).max(3).min(15);

    // 渲染前确保底部有足够空间
    let needed = header_rows + view_size as u16 + indicator_rows;
    let pos_row = cursor::position()?.1;
    let start_row = if pos_row + needed > term_rows {
        let extra = (pos_row + needed).saturating_sub(term_rows);
        for _ in 0..extra {
            queue!(stdout, Print("\n"))?;
        }
        stdout.flush()?;
        pos_row.saturating_sub(extra)
    } else {
        pos_row
    };

    loop {
        let filtered: Vec<_> = filter::filter_items(items, &filter_input);
        if cursor_pos >= filtered.len() && !filtered.is_empty() {
            cursor_pos = filtered.len() - 1;
        }
        // 保持滚动窗口跟随光标
        if cursor_pos < scroll_offset {
            scroll_offset = cursor_pos;
        } else if cursor_pos >= scroll_offset + view_size {
            scroll_offset = cursor_pos + 1 - view_size;
        }

        queue!(stdout, cursor::MoveTo(0, start_row))?;
        render(stdout, title, &filtered, cursor_pos, scroll_offset, view_size, &mode, &filter_input)?;
        stdout.flush()?;

        if let Event::Key(key) = event::read()? {
            match mode {
                Mode::Normal => {
                    match (key.code, key.modifiers) {
                        (KeyCode::Char('j'), KeyModifiers::NONE) | (KeyCode::Down, _) => {
                            if cursor_pos + 1 < filtered.len() {
                                cursor_pos += 1;
                            }
                            pending_g = false;
                        }
                        (KeyCode::Char('k'), KeyModifiers::NONE) | (KeyCode::Up, _) => {
                            cursor_pos = cursor_pos.saturating_sub(1);
                            pending_g = false;
                        }
                        (KeyCode::Char('g'), KeyModifiers::NONE) => {
                            if pending_g {
                                cursor_pos = 0;
                                pending_g = false;
                            } else {
                                pending_g = true;
                            }
                        }
                        (KeyCode::Char('G'), KeyModifiers::SHIFT) => {
                            if !filtered.is_empty() {
                                cursor_pos = filtered.len() - 1;
                            }
                            pending_g = false;
                        }
                        (KeyCode::Char('f'), KeyModifiers::NONE) => {
                            mode = Mode::Filter;
                            filter_cursor = filter_input.len();
                            pending_g = false;
                        }
                        (KeyCode::Enter, _) => {
                            clear_lines(stdout, start_row, &filtered, &mode)?;
                            return Ok(filtered.get(cursor_pos).map(|s| s.to_string()));
                        }
                        (KeyCode::Esc, _) | (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                            clear_lines(stdout, start_row, &filtered, &mode)?;
                            return Ok(None);
                        }
                        _ => {
                            pending_g = false;
                        }
                    }
                }
                Mode::Filter => {
                    match (key.code, key.modifiers) {
                        (KeyCode::Char(c), KeyModifiers::NONE) | (KeyCode::Char(c), KeyModifiers::SHIFT) => {
                            filter_input.insert(filter_cursor, c);
                            filter_cursor += 1;
                            cursor_pos = 0;
                        }
                        (KeyCode::Backspace, _) => {
                            if filter_cursor > 0 {
                                filter_input.remove(filter_cursor - 1);
                                filter_cursor -= 1;
                            }
                            cursor_pos = 0;
                        }
                        (KeyCode::Left, _) => {
                            filter_cursor = filter_cursor.saturating_sub(1);
                        }
                        (KeyCode::Right, _) => {
                            if filter_cursor < filter_input.len() {
                                filter_cursor += 1;
                            }
                        }
                        (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
                            kill_slot = filter_input[..filter_cursor].to_string();
                            filter_input.drain(..filter_cursor);
                            filter_cursor = 0;
                            cursor_pos = 0;
                        }
                        (KeyCode::Char('k'), KeyModifiers::CONTROL) => {
                            kill_slot = filter_input[filter_cursor..].to_string();
                            filter_input.truncate(filter_cursor);
                            cursor_pos = 0;
                        }
                        (KeyCode::Char('y'), KeyModifiers::CONTROL) => {
                            filter_input.insert_str(filter_cursor, &kill_slot);
                            filter_cursor += kill_slot.len();
                        }
                        (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                            filter_input.clear();
                            filter_cursor = 0;
                            cursor_pos = 0;
                        }
                        (KeyCode::Up, _) | (KeyCode::Char('p'), KeyModifiers::CONTROL) => {
                            cursor_pos = cursor_pos.saturating_sub(1);
                        }
                        (KeyCode::Down, _) | (KeyCode::Char('n'), KeyModifiers::CONTROL) => {
                            if cursor_pos + 1 < filtered.len() {
                                cursor_pos += 1;
                            }
                        }
                        (KeyCode::Enter, _) => {
                            clear_lines(stdout, start_row, &filtered, &mode)?;
                            return Ok(filtered.get(cursor_pos).map(|s| s.to_string()));
                        }
                        (KeyCode::Esc, _) => {
                            mode = Mode::Normal;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

fn render(stdout: &mut impl Write, title: &str, items: &[&String], cursor: usize, scroll_offset: usize, view_size: usize, mode: &Mode, filter: &str) -> io::Result<()> {
    queue!(stdout, cursor::MoveToColumn(0), terminal::Clear(ClearType::FromCursorDown))?;

    match mode {
        Mode::Normal => {
            queue!(stdout, Print(title), cursor::MoveToNextLine(1))?;
            queue!(stdout, Print("j/k:移动 gg/G:首/尾 f:搜索 Enter:选择 Esc:取消"), cursor::MoveToNextLine(2))?;
        }
        Mode::Filter => {
            queue!(stdout, Print(format!("{} [Filter: {}]", title, filter)), cursor::MoveToNextLine(1))?;
            queue!(stdout, Print("输入搜索 ↑↓:移动 Ctrl-U/K:删除 Esc:退出搜索 Enter:选择"), cursor::MoveToNextLine(2))?;
        }
    }

    let visible = items.iter().skip(scroll_offset).take(view_size);
    for (i, item) in visible.enumerate() {
        let abs_idx = scroll_offset + i;
        if abs_idx == cursor {
            queue!(stdout, SetForegroundColor(Color::Green), Print(format!("> {}", item)), ResetColor, cursor::MoveToNextLine(1))?;
        } else {
            queue!(stdout, Print(format!("  {}", item)), cursor::MoveToNextLine(1))?;
        }
    }

    // 滚动指示器
    if items.len() > view_size {
        queue!(stdout, cursor::MoveToNextLine(1))?;
        let indicator = format!("[{}/{}]", cursor + 1, items.len());
        queue!(stdout, Print(indicator))?;
    }

    Ok(())
}

fn clear_lines(stdout: &mut impl Write, start_row: u16, _items: &[&String], _mode: &Mode) -> io::Result<()> {
    queue!(stdout, cursor::MoveTo(0, start_row))?;
    queue!(stdout, terminal::Clear(ClearType::FromCursorDown))?;
    stdout.flush()?;
    Ok(())
}
