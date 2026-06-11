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

    let start_row = cursor::position()?.1;

    loop {
        let filtered: Vec<_> = filter::filter_items(items, &filter_input);
        if cursor_pos >= filtered.len() && !filtered.is_empty() {
            cursor_pos = filtered.len() - 1;
        }

        queue!(stdout, cursor::MoveTo(0, start_row))?;
        render(stdout, title, &filtered, cursor_pos, &mode, &filter_input)?;
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

fn render(stdout: &mut impl Write, title: &str, items: &[&String], cursor: usize, mode: &Mode, filter: &str) -> io::Result<()> {
    queue!(stdout, terminal::Clear(ClearType::FromCursorDown))?;

    match mode {
        Mode::Normal => {
            queue!(stdout, Print(format!("{}\n", title)))?;
        }
        Mode::Filter => {
            queue!(stdout, Print(format!("{} [Filter: {}]\n", title, filter)))?;
        }
    }

    for (i, item) in items.iter().take(10).enumerate() {
        if i == cursor {
            queue!(stdout, SetForegroundColor(Color::Green), Print("> "), Print(item), ResetColor, Print("\n"))?;
        } else {
            queue!(stdout, Print("  "), Print(item), Print("\n"))?;
        }
    }

    Ok(())
}

fn clear_lines(stdout: &mut impl Write, start_row: u16, _items: &[&String], _mode: &Mode) -> io::Result<()> {
    queue!(stdout, cursor::MoveTo(0, start_row))?;
    queue!(stdout, terminal::Clear(ClearType::FromCursorDown))?;
    stdout.flush()?;
    Ok(())
}
