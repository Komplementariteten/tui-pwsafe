use crate::contracts::UiWidgetVm;
use crate::ui::list_content::ContentList;
use crate::ui::password_input::PasswordWidget;
use crate::SafeModel;
use crossterm::event::Event::{Key, Mouse};
use crossterm::event::{
    DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEvent, KeyModifiers, MouseButton,
    MouseEventKind,
};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{event, execute};
use std::error::Error;
use std::io;
use std::time::{Duration, Instant};
use tui::backend::{Backend, CrosstermBackend};
use tui::{Frame, Terminal};

#[derive(Eq, PartialEq)]
enum View {
    PasswordDialog,
    ListStoreContent,
    CurrentView,
}

struct App<B: Backend> {
    pub should_quit: bool,
    pub active_mv: Box<dyn UiWidgetVm<B>>,
    pub next_view: View,
    pub current_view: View,
}

impl<B: Backend> App<B> {
    pub fn new() -> Self {
        App {
            active_mv: Box::new(PasswordWidget::new()),
            should_quit: false,
            next_view: View::PasswordDialog,
            current_view: View::PasswordDialog,
        }
    }
}

pub fn run(store: SafeModel) -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;
    let app = App::new();
    match run_app(&mut term, app, store) {
        Ok(_) => (),
        Err(e) => panic!("Error to run App: {:?}", e),
    };
    disable_raw_mode()?;
    execute!(
        term.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    term.show_cursor()?;
    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App<B>,
    mut model: SafeModel,
) -> io::Result<()> {
    let mut last_tick = Instant::now();

    // ~ 60 Hz
    let tick_rate = Duration::from_millis(17);
    loop {
        terminal.draw(|f| draw(f, &mut app, &mut model))?;
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if event::poll(timeout)? {
            if let Ok(ev) = event::read() {
                match ev {
                    Key(key) => match key {
                        KeyEvent {
                            code: KeyCode::Esc,
                            kind: _,
                            state: _,
                            modifiers: KeyModifiers::NONE,
                        } => app.should_quit = true,
                        _ => app.active_mv.capture_key(key),
                    },
                    Mouse(mouse) => {
                        if mouse.kind == MouseEventKind::Down(MouseButton::Left) {
                            // print!("{:?}", mouse)
                        }
                    }
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.active_mv.update_model(&mut model);
            last_tick = Instant::now();
        }

        if app.should_quit {
            return Ok(());
        }
    }
}

fn next_view(cur: &View) -> View {
    match cur {
        View::PasswordDialog => View::ListStoreContent,
        _ => View::CurrentView,
    }
}

fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App<B>, model: &mut SafeModel) {
    let size = f.size();

    if app.active_mv.is_done() {
        app.next_view = next_view(&app.current_view);
    }

    match app.next_view {
        View::PasswordDialog => {
            app.active_mv = Box::new(PasswordWidget::new());
            app.next_view = View::CurrentView;
        }
        View::ListStoreContent => {
            app.active_mv = Box::new(ContentList::new(model));
            app.next_view = View::CurrentView;
        }
        _ => {}
    };

    app.active_mv.draw(f, size);
}
