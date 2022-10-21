use crate::contracts::UiWidgetVm;
use crate::SafeModel;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use rs_pwsafe::pwsdb::record::DbRecord;
use std::collections::HashSet;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::Spans;
use tui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph};
use tui::Frame;

const HELP_TEXT: &str = "Press <Ctrl-p> to copy password, <Ctrl-u> for username";

pub struct ContentList {
    groups: StatefulList<String>,
    entries: StatefulList<String>,
    active_group_name: String,
    active_entry_name: Option<String>,
    active_entry: Option<DbRecord>,
    select_group: bool,
    selection: Option<String>,
    help_text: String,
    search_text: String,
}

struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T: Ord> StatefulList<T> {
    fn with_hs(items: HashSet<T>) -> StatefulList<T> {
        let mut vec: Vec<T> = Vec::from_iter(items);
        vec.sort();
        StatefulList {
            state: ListState::default(),
            items: vec,
        }
    }

    fn with_vec(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

impl<B: Backend> UiWidgetVm<B> for ContentList {
    fn capture_key(&mut self, key: KeyEvent) {
        match key {
            KeyEvent {
                kind: _,
                state: _,
                code: KeyCode::Char('p'),
                modifiers: KeyModifiers::CONTROL,
            } => {
                if let Some(item) = &self.active_entry {
                    self.selection = item.password();
                    self.help_text = "Password copied".to_string();
                }
            }
            KeyEvent {
                kind: _,
                state: _,
                code: KeyCode::Char('u'),
                modifiers: KeyModifiers::CONTROL,
            } => {
                if let Some(item) = &self.active_entry {
                    self.selection = item.username();
                    self.help_text = "Username copied".to_string();
                }
            }
            KeyEvent {
                kind: _,
                state: _,
                code: _,
                modifiers: KeyModifiers::NONE,
            } => match key.code {
                KeyCode::Tab => self.select_group = !self.select_group,
                KeyCode::Down => {
                    self.search_text = String::new();
                    if self.select_group {
                        self.groups.next();
                        if let Some(selected) = self.groups.state.selected() {
                            self.active_group_name = self.groups.items[selected].clone();
                        }
                    } else {
                        self.entries.next();
                        if let Some(selected) = self.entries.state.selected() {
                            self.active_entry_name = Some(self.entries.items[selected].clone());
                        }
                    }
                }
                KeyCode::Up => {
                    self.search_text = String::new();
                    if self.select_group {
                        self.groups.previous();
                        if let Some(selected) = self.groups.state.selected() {
                            self.active_group_name = self.groups.items[selected].clone();
                        }
                    } else {
                        self.entries.previous();
                        if let Some(selected) = self.entries.state.selected() {
                            self.active_entry_name = Some(self.entries.items[selected].clone());
                        }
                    }
                }
                KeyCode::Char(char) => {
                    self.search_text.push(char);
                    if let Some(pos) = self
                        .entries
                        .items
                        .iter()
                        .position(|entry| entry.contains(&self.search_text))
                    {
                        self.entries.state.select(Some(pos));
                        self.active_entry_name = Some(self.entries.items[pos].clone());
                    }
                }
                _ => {
                    print!("uncaptured key-code: {:?}", key);
                }
            },
            _ => {
                print!("uncaptured key: {:?}", key);
            }
        };
    }

    fn draw(&mut self, f: &mut Frame<B>, rec: Rect) {
        f.render_widget(Clear, rec);
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(60),
            ])
            .split(rec);

        // groups
        let items: Vec<ListItem> = self
            .groups
            .items
            .iter()
            .map(|it| {
                ListItem::new(it.as_str()).style(Style::default().fg(Color::White).bg(Color::Black))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Groups"))
            .highlight_style(
                Style::default()
                    .fg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");
        f.render_stateful_widget(list, chunks[0], &mut self.groups.state);

        // entries
        let items: Vec<ListItem> = self
            .entries
            .items
            .iter()
            .map(|it| {
                ListItem::new(it.as_str()).style(Style::default().fg(Color::White).bg(Color::Black))
            })
            .collect();

        let entire_list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Groups"))
            .highlight_style(
                Style::default()
                    .fg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

        f.render_stateful_widget(entire_list, chunks[1], &mut self.entries.state);

        // entrie
        if let Some(entry) = &self.active_entry {
            let sub_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
                .split(chunks[2]);

            let mut text = vec![Spans::from(format!(
                "{:12}: {}",
                "title",
                entry.title().unwrap()
            ))];
            if let Some(uname) = entry.username() {
                text.push(Spans::from(format!("{:12}: {}", "username", uname)))
            }
            if let Some(pw) = entry.password() {
                text.push(Spans::from(format!("{:12}: {}", "password", pw)))
            }
            let entrie_p = Paragraph::new(text)
                .style(Style::default().bg(Color::White).fg(Color::Black))
                .block(Block::default().borders(Borders::ALL).title("Entry"));
            f.render_widget(entrie_p, sub_layout[0]);

            let help = Paragraph::new(HELP_TEXT)
                .style(Style::default().bg(Color::Black).fg(Color::Gray))
                .block(Block::default().borders(Borders::ALL).title("Help"));
            f.render_widget(help, sub_layout[1]);
        }
    }

    fn update_model(&mut self, model: &mut SafeModel) {
        let items_in_group = model.by_group_name(self.active_group_name.as_str());
        if self.select_group {
            self.entries.items = items_in_group.iter().map(|&e| e.title().unwrap()).collect();
            self.entries.state.select(Some(0));
        }
        if let Some(name) = &self.active_entry_name {
            if let Some(&record) = items_in_group
                .iter()
                .find(|&&i| i.title().is_some() && i.title().unwrap().as_str().eq(name.as_str()))
            {
                self.active_entry = Some(record.clone());
            }
            self.active_entry_name = None;
        }

        if let Some(content) = &self.selection {
            model.to_clipboard(content.as_str());
            self.help_text.push('*');
            self.selection = None;
        }
    }

    fn is_done(&self) -> bool {
        return false;
    }
}

impl ContentList {
    pub fn new(model: &SafeModel) -> Self {
        let groups = model.groups();
        let item_list = StatefulList::with_hs(groups);
        let entries: Vec<String> = model
            .by_group_name(item_list.items.first().unwrap().as_str())
            .iter()
            .map(|&e| e.title().unwrap())
            .collect();
        ContentList {
            search_text: String::new(),
            help_text: HELP_TEXT.to_string(),
            selection: None,
            groups: item_list,
            select_group: true,
            active_group_name: String::new(),
            active_entry_name: None,
            active_entry: None,
            entries: StatefulList::with_vec(entries),
        }
    }
}
