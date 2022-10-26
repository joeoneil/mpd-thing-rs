#![allow(dead_code)]

use std::io;
use std::io::Read;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use crate::containers::{BasicContainer, BasicWidget, Container, HSplitContainer, MetaContainer, RootContainer};

mod containers;

fn main() {
    let stdout = io::stdout().into_raw_mode().unwrap();
    let stdout = MouseTerminal::from(stdout);
    let backend = tui::backend::TermionBackend::new(stdout);
    let mut terminal = tui::Terminal::new(backend).unwrap();

    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();

    let mut root_container = RootContainer::new();

    root_container.set_child(0,
        Box::new(HSplitContainer::new(
            Box::new(BasicContainer::new(
                Box::new(BasicWidget::new("Hello".to_string(), "Lorem Ipsum".to_string()))
            )),
            Box::new(BasicContainer::new(
                Box::new(BasicWidget::new("world!".to_string(), "Dolor sit amet".to_string()))
            )),
            0.5
        ))
    );

    loop {
        terminal.draw(|f| {
            root_container.set_bounds(f.size());
            root_container.draw(f);
        }).unwrap();
    }

    terminal.show_cursor().unwrap();
}