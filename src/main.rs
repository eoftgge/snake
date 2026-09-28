use std::io::{self, BufWriter, Stdout};
use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::{cursor, execute, terminal};

use snake::game::difficulty::Difficulty;
use snake::game::snake::Direction;
use snake::game::{Game, TickResult};
use snake::render::Renderer;

struct TerminalGuard;

impl TerminalGuard {
    fn new() -> io::Result<Self> {
        terminal::enable_raw_mode()?;
        execute!(
            io::stdout(),
            terminal::EnterAlternateScreen,
            terminal::Clear(terminal::ClearType::All),
            cursor::Hide,
        )?;
        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = execute!(io::stdout(), cursor::Show, terminal::LeaveAlternateScreen);
        let _ = terminal::disable_raw_mode();
    }
}

#[cfg(windows)]
fn install_close_handler() {
    use windows_sys::Win32::Foundation::BOOL;
    use windows_sys::Win32::System::Console::{
        SetConsoleCtrlHandler, CTRL_CLOSE_EVENT, CTRL_LOGOFF_EVENT, CTRL_SHUTDOWN_EVENT,
    };
    use windows_sys::Win32::System::Threading::{GetCurrentProcess, TerminateProcess};

    unsafe extern "system" fn handler(event: u32) -> BOOL {
        match event {
            CTRL_CLOSE_EVENT | CTRL_LOGOFF_EVENT | CTRL_SHUTDOWN_EVENT => {
                TerminateProcess(GetCurrentProcess(), 0);
                1
            }
            _ => 0,
        }
    }

    unsafe {
        SetConsoleCtrlHandler(Some(handler), 1);
    }
}

#[cfg(not(windows))]
fn install_close_handler() {}

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    Menu,
    Playing,
    Over { won: bool },
}

enum Action {
    Continue,
    Quit,
}

struct App {
    game: Game,
    renderer: Renderer<BufWriter<Stdout>>,
    state: State,
    difficulty: Difficulty,
    last_tick: Instant,
    width: usize,
    height: usize,
}

impl App {
    fn new(width: usize, height: usize) -> io::Result<Self> {
        let mut app = Self {
            game: Game::new(width, height),
            renderer: Renderer::new(BufWriter::new(io::stdout()), width, height),
            state: State::Menu,
            difficulty: Difficulty::Normal,
            last_tick: Instant::now(),
            width,
            height,
        };
        app.redraw_all()?;
        Ok(app)
    }

    fn tick_len(&self) -> Duration {
        self.difficulty.tick(self.game.score())
    }

    fn poll_timeout(&self) -> Duration {
        match self.state {
            State::Playing => self.tick_len().saturating_sub(self.last_tick.elapsed()),
            State::Menu | State::Over { .. } => Duration::from_secs(1),
        }
    }

    fn on_key(&mut self, key: KeyEvent) -> io::Result<Action> {
        let ctrl_c =
            key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL);
        if ctrl_c || matches!(key.code, KeyCode::Esc | KeyCode::Char('q')) {
            return Ok(Action::Quit);
        }

        match self.state {
            State::Menu => {
                let difficulty = match key.code {
                    KeyCode::Char('1') => Some(Difficulty::Easy),
                    KeyCode::Char('2') => Some(Difficulty::Normal),
                    KeyCode::Char('3') => Some(Difficulty::Hard),
                    _ => None,
                };
                if let Some(d) = difficulty {
                    self.difficulty = d;
                    self.start()?;
                }
            }
            State::Playing => {
                if let Some(dir) = direction_for(key.code) {
                    self.game.snake.turn(dir);
                }
            }
            State::Over { .. } => match key.code {
                KeyCode::Char('r') => self.start()?,
                KeyCode::Char('m') => {
                    self.state = State::Menu;
                    self.redraw_all()?;
                }
                _ => {}
            },
        }
        Ok(Action::Continue)
    }

    fn on_tick(&mut self) -> io::Result<()> {
        if self.state != State::Playing || self.last_tick.elapsed() < self.tick_len() {
            return Ok(());
        }
        self.last_tick = Instant::now();

        match self.game.tick() {
            TickResult::Running => self.renderer.draw(&self.game)?,
            TickResult::Lost => self.finish(false)?,
            TickResult::Won => {
                self.renderer.draw(&self.game)?;
                self.finish(true)?;
            }
        }
        Ok(())
    }

    fn finish(&mut self, won: bool) -> io::Result<()> {
        self.state = State::Over { won };
        self.draw_overlay()
    }

    fn start(&mut self) -> io::Result<()> {
        self.game = Game::new(self.width, self.height);
        self.state = State::Playing;
        self.last_tick = Instant::now();
        self.redraw_all()
    }

    fn redraw_all(&mut self) -> io::Result<()> {
        execute!(io::stdout(), terminal::Clear(terminal::ClearType::All))?;
        self.renderer.invalidate();
        self.renderer.draw(&self.game)?;
        self.draw_overlay()
    }

    fn draw_overlay(&mut self) -> io::Result<()> {
        let lines = match self.state {
            State::Playing => return Ok(()),
            State::Menu => vec![
                "SNAKE".to_string(),
                String::new(),
                "1 - easy".to_string(),
                "2 - normal".to_string(),
                "3 - hardeeest".to_string(),
                String::new(),
                "Q - quit".to_string(),
            ],
            State::Over { won } => vec![
                if won { "WIN!" } else { "GAME OVER" }.to_string(),
                format!("Score: {}", self.game.score()),
                format!("Difficulty: {}", self.difficulty.name()),
                String::new(),
                "R - restart, M - menu".to_string(),
                "Q - quit".to_string(),
            ],
        };
        self.renderer.draw_message(&lines)
    }
}

fn direction_for(code: KeyCode) -> Option<Direction> {
    match code {
        KeyCode::Up | KeyCode::Char('w') => Some(Direction::Up),
        KeyCode::Down | KeyCode::Char('s') => Some(Direction::Down),
        KeyCode::Left | KeyCode::Char('a') => Some(Direction::Left),
        KeyCode::Right | KeyCode::Char('d') => Some(Direction::Right),
        _ => None,
    }
}

fn main() -> io::Result<()> {
    install_close_handler();

    let (cols, rows) = terminal::size()?;
    let width = cols as usize / 2;
    let height = rows as usize;

    let _guard = TerminalGuard::new()?;
    let mut app = App::new(width, height)?;

    loop {
        if event::poll(app.poll_timeout())? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    if let Action::Quit = app.on_key(key)? {
                        break;
                    }
                }
                Event::Resize(_, _) => app.redraw_all()?,
                _ => {}
            }
        }
        app.on_tick()?;
    }

    Ok(())
}