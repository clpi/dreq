use std::{io, error::Error};
use crossterm::{
    event::{KeyCode, KeyEvent},
    terminal::{enable_raw_mode, EnterAlternateScreen}
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Tabs, Borders, Block, List, Paragraph},
};


pub struct RequestView {

}
