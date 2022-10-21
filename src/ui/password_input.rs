use crate::contracts::UiWidgetVm;
use crate::SafeModel;
use crossterm::event;
use crossterm::event::{KeyCode, KeyEvent};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::widgets::{Block, Borders, Clear, Paragraph};
use tui::Frame;
use unicode_width::UnicodeWidthStr;

const HELP_TEXT: &str = "Left-Ctrl to unhide";

pub struct PasswordWidget {
    key_input: String,
    error: String,
    has_error: bool,
    enter_done: bool,
    hide_pw: bool,
    is_done: bool,
}

impl<B: Backend> UiWidgetVm<B> for PasswordWidget {
    fn capture_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Backspace => {
                self.key_input.pop();
            }
            KeyCode::Char(c) => {
                self.key_input.push(c);
            }
            KeyCode::Enter => {
                self.enter_done = true;
            }
            _ => {
                self.hide_pw = true;
            }
        };
        match key {
            event::KeyEvent {
                code: _,
                modifiers: event::KeyModifiers::CONTROL,
                ..
            } => {
                self.hide_pw = false;
            }
            _ => {
                self.hide_pw = true;
            }
        };
    }

    fn draw(&mut self, f: &mut Frame<B>, rec: Rect) {
        f.render_widget(Clear, rec);
        let pw_display = if self.hide_pw {
            self.key_input.clone().chars().map(|_| "*").collect()
        } else {
            self.key_input.clone()
        };

        let input = Paragraph::new(pw_display).block(
            Block::default()
                .title("Enter Password")
                .borders(Borders::ALL),
        );
        let popup = self.centered_rect(60, 40, rec);
        let chunks = Layout::default()
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(popup);
        f.render_widget(input, chunks[0]);

        let helptext = Paragraph::new(if !self.has_error {
            HELP_TEXT
        } else {
            self.error.as_str()
        })
        .block(Block::default().title("Help").borders(Borders::ALL));
        f.render_widget(helptext, chunks[1]);
        f.set_cursor(
            chunks[0].x + self.key_input.width() as u16 + 1,
            chunks[0].y + 1,
        );
    }

    fn update_model(&mut self, model: &mut SafeModel) {
        if !self.enter_done {
            return;
        }
        if let Err(e) = model.unlock(self.key_input.as_str()) {
            self.has_error = true;
            self.error = format!("Error in unlock pwsafe: {}", e);
        } else {
            println!("unlocked");
            self.is_done = true;
        }
    }

    fn is_done(&self) -> bool {
        self.is_done
    }
}

impl PasswordWidget {
    pub fn new() -> Self {
        PasswordWidget {
            enter_done: false,
            has_error: false,
            error: String::new(),
            hide_pw: true,
            key_input: String::new(),
            is_done: false,
        }
    }

    /// helper function to create a centered rect using up certain percentage of the available rect `r`
    fn centered_rect(&self, percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage((100 - percent_y) / 2),
                    Constraint::Percentage(percent_y),
                    Constraint::Percentage((100 - percent_y) / 2),
                ]
                .as_ref(),
            )
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage((100 - percent_x) / 2),
                    Constraint::Percentage(percent_x),
                    Constraint::Percentage((100 - percent_x) / 2),
                ]
                .as_ref(),
            )
            .split(popup_layout[1])[1]
    }
}
