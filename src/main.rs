#![allow(dead_code)]
#![allow(unused_variables)]

use std::{io, thread};
use std::sync::mpsc;
use std::time::Duration;
use termion::event::*;
use termion::input::{MouseTerminal, TermRead};
use termion::raw::{IntoRawMode};
use tui::backend::TermionBackend;
use tui::Frame;
use tui::layout::{Alignment, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Paragraph, Wrap};
use crate::containers::{BasicContainer, BasicWidget, Container, HSplitContainer, RootContainer, VSplitContainer, WStyleOpt};

mod containers;

enum ThingEvent {
    Tick,
    Key(Key),
}

enum InputMode {
    Normal(ContainerStack),
    Select(ContainerStack),
    Insert(ContainerStack),
}

impl Into<ContainerStack> for InputMode {
    fn into(self) -> ContainerStack {
        match self {
            InputMode::Normal(stack) => stack,
            InputMode::Select(stack) => stack,
            InputMode::Insert(stack) => stack,
        }
    }
}

struct ContainerStack{
    stack: Vec<u8>,
    root: Box<dyn Container>,
}

impl ContainerStack {
    fn new(root: Box<dyn Container>) -> Self {
        Self {
            stack: Vec::new(),
            root,
        }
    }

    fn push(&mut self, id: u8) {
        self.stack.push(id);
    }

    fn current(&self) -> Option<&Box<dyn Container>> {
        self.stack.iter().fold(Some(&self.root), |container, id| {
            match container {
                Some(container) => container.get_child(*id),
                None => None,
            }
        })
    }

    fn current_mut(&mut self) -> Option<&mut Box<dyn Container>> {
        self.stack.iter().fold(Some(&mut self.root), |container, id| {
            match container {
                Some(container) => container.get_child_mut(*id),
                None => None,
            }
        })
    }

    fn set_selected_style(&mut self) {
        if let Some(container) = self.current_mut() {
            container.set_override_style(WStyleOpt::default().set_border_style(Style::default().fg(tui::style::Color::Yellow)));
        }
    }

    fn unset_selected_style(&mut self) {
        if let Some(container) = self.current_mut() {
            container.unset_override_style();
        }
    }

    fn set_child_selected_style(&mut self, index: u8) {
        if let Some(container) = self.current_mut() {
            if let Some(child) = container.get_child_mut(index) {
                child.set_override_style(WStyleOpt::default().set_border_style(Style::default().fg(tui::style::Color::Yellow)));
            }
        }
    }

    fn unset_child_selected_style(&mut self, index: u8) {
        if let Some(container) = self.current_mut() {
            if let Some(child) = container.get_child_mut(index) {
                child.unset_override_style();
            }
        }
    }

    fn focus_down(&mut self, index: u8) {
        self.unset_selected_style();
        if self.current_mut().unwrap().has_children() {
            if let Some(child) = self.current_mut().unwrap().get_child_mut(index) {
                self.push(index);
            }
        }
        if let Some(container) = self.current_mut() {
            if container.has_children() {
                self.set_child_selected_style(0);
            } else {
                self.set_selected_style();
            }
        }
    }

    fn focus_up(&mut self) -> u8 {
        self.unset_selected_style();
        let ret = if self.stack.len() > 1 {
            // prevents popping root container
            self.stack.pop().unwrap()
        } else {
            0
        };
        self.set_child_selected_style(ret);
        ret
    }

    fn focus_shift(&mut self, cur_index: u8, left: bool) -> bool{
        self.unset_selected_style();
        if let Some(container) = self.current_mut() {
            let new_index = if left {
                if cur_index == 0 { 0 } else { cur_index - 1 }
            } else {
                cur_index + 1
            };
            return if let Some(child) = container.get_child_mut(new_index) {
                self.set_child_selected_style(new_index);
                true
            } else {
                self.set_selected_style();
                false
            }
        }
        false
    }

    fn current_has_children(&self) -> bool {
        if let Some(container) = self.current() {
            container.has_children()
        } else {
            false
        }
    }
}

fn main() {
    let stdin = io::stdin();

    let stdout = io::stdout().into_raw_mode().unwrap();
    let stdout = MouseTerminal::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = tui::Terminal::new(backend).unwrap();

    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();

    let mut root_container = RootContainer::new();

    let left_box = String::from("Left");
    let right_box = String::from("Right");
    let top_box = String::from("Top");
    let bottom_box = String::from("");

    root_container.set_child(0, Box::new(HSplitContainer::new(
        Box::new(VSplitContainer::new(
            Box::new(BasicContainer::new(
                Box::new(BasicWidget::new("Top Left".to_string(), "Some Text".to_string())),
            )),
            Box::new(BasicContainer::new(
                Box::new(BasicWidget::new("Bottom Left".to_string(), "Some Text".to_string())),

            )),
            0.5
        )),
            Box::new(VSplitContainer::new(
                Box::new(BasicContainer::new(
                    Box::new(BasicWidget::new("Lorem Ipsum".to_string(), top_box))
                )),
                Box::new(HSplitContainer::new(
                    Box::new(BasicContainer::new(
                        Box::new(BasicWidget::new("Infinite Possibility".to_string(), bottom_box))
                    )),
                    Box::new(BasicContainer::new(
                        Box::new(BasicWidget::new("Death Gripsum".to_string(), right_box))
                    )),
                    0.75
                )),
                0.15,
            )),
            0.15
        ))
    );
    let mut stack = ContainerStack::new(Box::new(root_container));
    stack.push(0);
    let mut input_mode = InputMode::Normal(stack);
    let mut selection_index = 0 as u8;
    let mut menu_selection_index = 0 as u8;

    let events = events(Duration::from_micros(1000000 / 60));

    fn draw(stack: &ContainerStack, f: &mut Frame<containers::F>, bottom_text: &str) {
        let area = f.size();
        stack.root.draw(f, Rect::new(0, 0, area.width, area.height - 1));
        let bottom_bar = Paragraph::new(bottom_text)
            .block(Block::default().borders(Borders::NONE))
            .style(Style::default().fg(Color::White))
            .wrap(Wrap { trim: true });
        f.render_widget(bottom_bar, Rect::new(area.x, area.height - 1, area.width, 1));
    }

    loop {
        match events.recv().unwrap() {
            ThingEvent::Tick => {
                terminal.draw(|f| {
                    match &input_mode {
                        InputMode::Normal(container_hierarchy) => {
                            draw(container_hierarchy, f, "Normal Mode | Press 'q' to quit | Press 'i' to enter insert mode");
                        },
                        InputMode::Select(container_hierarchy) => {
                            draw(container_hierarchy, f, "Select Mode | Press 'q' to quit | Press 'c' to exit mode | Use arrow keys to navigate | Press ENTER to replace component");
                        },
                        InputMode::Insert(container_heirarchy) => {
                            draw(container_heirarchy, f, "Insert Mode | Press 'q' to quit | Press 'c' to exit mode | Use arrow keys to navigate | Press ENTER to insert component");
                            let area = f.size();
                            let context_menu = Paragraph::new("Insert Mode")
                                .block(Block::default().borders(tui::widgets::Borders::ALL))
                                .style(Style::default().fg(tui::style::Color::White))
                                .wrap(Wrap { trim: true })
                                .alignment(tui::layout::Alignment::Center);
                            // create a list of possible containers and widgets
                            let items = vec!["Horizontal Split Container", "Vertical Split Container", "Basic Widget"];
                            let mut index = 0;
                            let menu_items = items.iter().map(|text| {
                                let out = Paragraph::new(*text)
                                    .block(Block::default().borders(tui::widgets::Borders::NONE))
                                    .style(
                                        if index == menu_selection_index as usize {
                                            Style::default().fg(tui::style::Color::Yellow)
                                        } else {
                                            Style::default().fg(tui::style::Color::White)
                                        }
                                    )
                                    .wrap(Wrap { trim: true })
                                    .alignment(tui::layout::Alignment::Center);
                                index += 1;
                                return out;
                            }).collect::<Vec<Paragraph>>();
                            f.render_widget(context_menu, Rect::new(area.width / 2 - 10, area.height / 2 - 2, 20, 2 + items.len() as u16));
                            index = 0;
                            for item in menu_items {
                                f.render_widget(item, Rect::new(area.width / 2 - 10, area.height / 2 - 1 + index as u16, 20, 1));
                                index += 1;
                            }
                        }
                    }
                }).unwrap();
            },
            ThingEvent::Key(key) => {
                match input_mode {
                    InputMode::Normal(mut x) => {
                        match key {
                            Key::Char('q') => break,
                            Key::Char('i') => {
                                x.set_selected_style();
                                input_mode = InputMode::Select(x);
                                continue;
                            }
                            _ => {}
                        }
                        input_mode = InputMode::Normal(x);
                    }
                    InputMode::Select(mut x) => {
                        x.set_child_selected_style(selection_index);
                        match key {
                            Key::Char('q') => break,
                            Key::Char('c') => {
                                x.root.unset_override_style();
                                input_mode = InputMode::Normal(x);
                                continue;
                            }
                            Key::Down => {
                                x.focus_down(selection_index);
                                selection_index = 0;
                            }
                            Key::Up => {
                                selection_index = x.focus_up();
                            }
                            Key::Left => {
                                if x.focus_shift(selection_index, true) {
                                    selection_index = selection_index.saturating_sub(1);
                                }
                            }
                            Key::Right => {
                                if x.focus_shift(selection_index, false) {
                                    selection_index = selection_index.saturating_add(1);
                                }
                            }
                            Key::Delete => {
                                x.current_mut().unwrap().set_child(selection_index as usize, Box::new(BasicContainer::default()));
                            }
                            Key::Char('r') => {
                                x.root.set_child(0, Box::new(BasicContainer::default()));
                                x.stack = vec![0];
                            }
                            Key::Char('\n') => {
                                menu_selection_index = 0;
                                input_mode = InputMode::Insert(x);
                                continue;
                            }
                            _ => {}
                        }
                        input_mode = InputMode::Select(x);
                    }
                    InputMode::Insert(mut x) => {
                        match key {
                            Key::Char('q') => break,
                            Key::Char('c') => {
                                input_mode = InputMode::Select(x);
                                selection_index = 0;
                                continue;
                            }
                            Key::Down => {
                                menu_selection_index = menu_selection_index.saturating_add(1);
                            }
                            Key::Up => {
                                menu_selection_index = menu_selection_index.saturating_sub(1);
                            }
                            Key::Char('\n') => {
                                match menu_selection_index {
                                    0 => {
                                        x.current_mut().unwrap().set_child(selection_index as usize, Box::new(HSplitContainer::default()));
                                    }
                                    1 => {
                                        x.current_mut().unwrap().set_child(selection_index as usize, Box::new(VSplitContainer::default()));
                                    }
                                    2 => {
                                        x.current_mut().unwrap().set_child(selection_index as usize, Box::new(BasicContainer::default()));
                                    }
                                    _ => {}
                                }
                                input_mode = InputMode::Insert(x);
                                selection_index = 0;
                                continue;
                            }
                            _ => {}
                        }
                        input_mode = InputMode::Insert(x);
                    }
                }
            }
        }
    }
    terminal.show_cursor().unwrap();
    terminal.clear().unwrap();
}

fn events(tick_rate: Duration) -> mpsc::Receiver<ThingEvent> {
    let (tx, rx) = mpsc::channel();
    let keys_tx = tx.clone();
    thread::spawn(move || {
        let stdin = io::stdin();
        for key in stdin.keys().flatten() {
            if let Err(err) = keys_tx.send(ThingEvent::Key(key)) {
                eprintln!("{}", err);
                return;
            }
        }
    });
    thread::spawn(move || loop {
        if let Err(err) = tx.send(ThingEvent::Tick) {
            eprintln!("{}", err);
            break;
        }
        thread::sleep(tick_rate);
    });
    return rx;
}