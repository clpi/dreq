use std::{io::{self, Read, Write}, thread, time, sync::mpsc::channel};
use crate::{
    mode::Mode,
    project::Project,
    event::Event,
};
use crossterm::{
    execute, write_ansi_code, queue, style, cursor,
    event::{
        self, DisableMouseCapture, EnableMouseCapture,
        Event as CEvent, KeyCode
    },
    terminal::{self,
        EnterAlternateScreen, LeaveAlternateScreen
    },
    Result as CTResult,
};
use tui::{backend::CrosstermBackend, Terminal};

#[derive(Debug)]
pub struct App<'a> {
    pub projects: Vec<Project>,
    pub modes: Vec<Mode<'a>>,
}

pub enum Msg {
    NoIp,
    Quit,
}


impl<'a> App<'a> {

    pub fn new() -> Self {
        Self {
            projects: Vec::new(),
            modes: Vec::new(),
        }
    }

    pub fn run<W>(&mut self, w: &mut W) -> CTResult<()>
        where
            W: Write,
    {
        terminal::enable_raw_mode()?;
        execute!(w, terminal::EnterAlternateScreen)?;
        while let Ok(msg) = self.refresh() {
            match msg {
                Msg::Quit => break,
                _ => {},
            }
            queue!(w,
                style::ResetColor,
                terminal::Clear(terminal::ClearType::All),
                cursor::Hide,
                cursor::MoveTo(1,1),
            )?;
            w.flush()?;
        };
        Ok(())
    }

    pub fn refresh(&mut self) -> io::Result<Msg> {
        Ok(Msg::NoIp)
    }

    pub fn ev_loop(&mut self) -> io::Result<()> {
        let mut stdout = io::stdout();
        terminal::enable_raw_mode().unwrap();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
        let backend = CrosstermBackend::new(stdout);
        let mut term = Terminal::new(backend)?;
        let (tx, rx) = channel();
        let tick_rate = time::Duration::from_millis(1);
        thread::spawn(move || {
            let mut last = time::Instant::now();
            loop {
                let timeout = tick_rate.checked_sub(last.elapsed())
                    .unwrap_or_else(|| time::Duration::from_secs(0));
                if event::poll(timeout).unwrap() {
                    if let CEvent::Key(key) = event::read().unwrap() {
                        tx.send(Event::Input(key)).unwrap();
                    }
                }
                if last.elapsed() >= tick_rate {
                    tx.send(Event::Tick).unwrap();
                    last = time::Instant::now();
                }
            }
        });
        term.clear()?;
        Ok(())
    }

    pub fn t() {
    }
}
