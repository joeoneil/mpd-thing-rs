use std::io::Stdout;
use termion::input::MouseTerminal;
use termion::raw::RawTerminal;
use tui::backend::{TermionBackend};
use tui::Frame;
use tui::style::{Style, Color};
use tui::layout::{Alignment, Rect};
use tui::text::{Span, Text};
use tui::widgets::{Block, Borders, Paragraph, Wrap};

pub type F = TermionBackend<MouseTerminal<RawTerminal<Stdout>>>;

pub trait Container {
    fn draw(&self, f: &mut Frame<F>, area: Rect);

    fn set_style(&mut self, style: WStyleOpt);

    fn set_override_style(&mut self, style: WStyleOpt);

    fn unset_override_style(&mut self);

    fn set_child(&mut self, index: usize, child: Box<dyn Container>);

    fn get_child(&self, index: u8) -> Option<&Box<dyn Container>>;

    fn get_child_mut(&mut self, index: u8) -> Option<&mut Box<dyn Container>>;

    fn set_widget(&mut self, widget: Box<dyn Widget>);

    fn get_widget(&self) -> Option<&Box<dyn Widget>>;

    fn get_widget_mut(&mut self) -> Option<&mut Box<dyn Widget>>;

    fn has_children(&self) -> bool;
}

pub trait Widget {
    fn draw(&self, f: &mut Frame<F>, area: Rect);

    fn get_style(&self) -> WStyle;

    fn set_override_style(&mut self, style: WStyleOpt);

    fn unset_override_style(&mut self);

    fn set_style(&mut self, style: WStyleOpt);
}


pub struct WStyle {
    title_style: Style,
    text_style: Style,
    border_style: Style,
}

impl Default for WStyle {
    fn default() -> Self {
        WStyle {
            title_style: Style::default().fg(Color::White).bg(Color::Black),
            text_style: Style::default().fg(Color::White).bg(Color::Black),
            border_style: Style::default().fg(Color::White).bg(Color::Black),
        }
    }
}

impl Clone for WStyle {
    fn clone(&self) -> Self {
        WStyle {
            title_style: self.title_style.clone(),
            text_style: self.text_style.clone(),
            border_style: self.border_style.clone(),
        }
    }
}

impl From<WStyleOpt> for WStyle {
    fn from(style: WStyleOpt) -> Self {
        let default = WStyle::default();
        WStyle {
            title_style: style.title_style.unwrap_or(default.title_style),
            text_style: style.text_style.unwrap_or(default.text_style),
            border_style: style.border_style.unwrap_or(default.border_style),
        }
    }
}

impl WStyle {
    pub fn new(title_style: Style, text_style: Style, border_style: Style) -> Self {
        WStyle {
            title_style,
            text_style,
            border_style,
        }
    }

    pub fn set(&mut self, style: WStyleOpt) -> &mut Self {
        if let Some(title_style) = style.title_style {
            self.title_style = title_style;
        }
        if let Some(text_style) = style.text_style {
            self.text_style = text_style;
        }
        if let Some(border_style) = style.border_style {
            self.border_style = border_style;
        }
        self
    }
}

pub struct WStyleOpt {
    title_style: Option<Style>,
    text_style: Option<Style>,
    border_style: Option<Style>,
}

impl Default for WStyleOpt {
    fn default() -> Self {
        WStyleOpt {
            title_style: None,
            text_style: None,
            border_style: None,
        }
    }
}

impl Clone for WStyleOpt {
    fn clone(&self) -> Self {
        WStyleOpt {
            title_style: self.title_style,
            text_style: self.text_style,
            border_style: self.border_style,
        }
    }
}

impl WStyleOpt {
    pub fn set_border_style(mut self, style: Style) -> Self {
        self.border_style = Some(style);
        self
    }
}


pub struct BasicContainer {
    child: Box<dyn Widget>,
}

impl Container for BasicContainer {
    fn draw(&self, f: &mut Frame<F>, area: Rect) {
        self.child.draw(f, area);
    }

    fn set_style(&mut self, style: WStyleOpt) {
        self.child.set_style(style);
    }

    fn set_override_style(&mut self, style: WStyleOpt) {
        self.child.set_override_style(style);
    }

    fn unset_override_style(&mut self) {
        self.child.unset_override_style();
    }

    fn set_child(&mut self, index: usize, child: Box<dyn Container>) {
    }

    fn get_child(&self, index: u8) -> Option<&Box<dyn Container>> {
        None
    }

    fn get_child_mut(&mut self, index: u8) -> Option<&mut Box<dyn Container>> {
        None
    }

    fn set_widget(&mut self, widget: Box<dyn Widget>) {
        self.child = widget;
    }

    fn get_widget(&self) -> Option<&Box<dyn Widget>> {
        Some(&self.child)
    }

    fn get_widget_mut(&mut self) -> Option<&mut Box<dyn Widget>> {
        Some(&mut self.child)
    }

    fn has_children(&self) -> bool {
        false
    }
}

impl Default for BasicContainer {
    fn default() -> Self {
        BasicContainer {
            child: Box::new(BasicWidget::default()),
        }
    }
}

impl BasicContainer {
    pub fn new(child: Box<dyn Widget>) -> Self {
        BasicContainer {
            child,
        }
    }
}


pub struct RootContainer {
    child: Box<dyn Container>,
}

impl Container for RootContainer {
    fn draw(&self, f: &mut Frame<F>, area: Rect) {
        if area.width < 2 || area.height < 2 {
            return;
        }
        self.child.draw(f, area);
    }

    fn set_style(&mut self, style: WStyleOpt) {
        self.child.set_style(style);
    }

    fn set_override_style(&mut self, style: WStyleOpt) {
        self.child.set_override_style(style);
    }

    fn unset_override_style(&mut self) {
        self.child.unset_override_style();
    }

    fn set_child(&mut self, index: usize, child: Box<dyn Container>) {
        match index {
            0 => self.child = child,
            _ => (),
        }
    }

    fn get_child(&self, index: u8) -> Option<&Box<dyn Container>> {
        match index {
            0 => Some(&self.child),
            _ => None,
        }
    }

    fn get_child_mut(&mut self, index: u8) -> Option<&mut Box<dyn Container>> {
        match index {
            0 => Some(&mut self.child),
            _ => None,
        }
    }

    fn set_widget(&mut self, widget: Box<dyn Widget>) {
    }

    fn get_widget(&self) -> Option<&Box<dyn Widget>> {
        None
    }

    fn get_widget_mut(&mut self) -> Option<&mut Box<dyn Widget>> {
        None
    }

    fn has_children(&self) -> bool {
        true
    }
}

impl Default for RootContainer {
    fn default() -> Self {
        RootContainer {
            child: Box::new(BasicContainer::default()),
        }
    }
}

impl RootContainer {
    pub fn new() -> Self {
        RootContainer {
            child: Box::new(BasicContainer::default()),
        }
    }

    pub fn into_dyn_container(self) -> Box<dyn Container> {
        Box::new(self)
    }
}


pub struct HSplitContainer {
    children: Vec<Box<dyn Container>>,
    split: f32,
}

impl Container for HSplitContainer {
    fn draw(&self, f: &mut Frame<F>, area: Rect) {
        let area = area;
        let mut split = self.split;
        if split < 0.0 {
            split = 0.0;
        } else if split > 1.0 {
            split = 1.0;
        }
        let split = split * area.width as f32;
        let split = split as u16;
        let left = Rect {
            x: area.x,
            y: area.y,
            width: split,
            height: area.height,
        };
        let right = Rect {
            x: area.x + split,
            y: area.y,
            width: area.width - split,
            height: area.height,
        };
        if left.width < 2 || right.width < 2 {
            if self.split > 0.5 {
                self.children[0].draw(f, area);
            } else {
                self.children[1].draw(f, area);
            }
        } else {
            self.children[0].draw(f, left);
            self.children[1].draw(f, right);
        }
    }

    fn set_style(&mut self, style: WStyleOpt) {
        self.children[0].set_style(style.clone());
        self.children[1].set_style(style);
    }

    fn set_override_style(&mut self, style: WStyleOpt) {
        self.children[0].set_override_style(style.clone());
        self.children[1].set_override_style(style);
    }

    fn unset_override_style(&mut self) {
        self.children[0].unset_override_style();
        self.children[1].unset_override_style();
    }

    fn set_child(&mut self, index: usize, child: Box<dyn Container>) {
        if index < self.children.len() {
            self.children[index] = child;
        }
    }

    fn get_child(&self, index: u8) -> Option<&Box<dyn Container>> {
        if index < self.children.len() as u8 {
            Some(&self.children[index as usize])
        } else {
            None
        }
    }

    fn get_child_mut(&mut self, index: u8) -> Option<&mut Box<dyn Container>> {
        if index < self.children.len() as u8 {
            Some(&mut self.children[index as usize])
        } else {
            None
        }
    }

    fn set_widget(&mut self, widget: Box<dyn Widget>) {
    }

    fn get_widget(&self) -> Option<&Box<dyn Widget>> {
        None
    }

    fn get_widget_mut(&mut self) -> Option<&mut Box<dyn Widget>> {
        None
    }

    fn has_children(&self) -> bool {
        true
    }
}

impl Default for HSplitContainer {
    fn default() -> Self {
        HSplitContainer {
            children: vec![Box::new(BasicContainer::default()), Box::new(BasicContainer::default())],
            split: 0.5,
        }
    }
}

impl HSplitContainer {
    pub fn new(left: Box<dyn Container>, right: Box<dyn Container>, split: f32) -> Self {
        HSplitContainer {
            children: vec![left, right],
            split,
        }
    }

    fn set_split(&mut self, split: f32) {
        self.split = split;
    }
}


pub struct VSplitContainer {
    children: Vec<Box<dyn Container>>,
    split: f32,
}

impl Container for VSplitContainer {
    fn draw(&self, f: &mut Frame<F>, area: Rect) {
        let area = area;
        let mut split = self.split;
        if split < 0.0 {
            split = 0.0;
        } else if split > 1.0 {
            split = 1.0;
        }
        let split = split * area.height as f32;
        let split = split as u16;
        let top = Rect {
            x: area.x,
            y: area.y,
            width: area.width,
            height: split,
        };
        let bottom = Rect {
            x: area.x,
            y: area.y + split,
            width: area.width,
            height: area.height - split,
        };
        if top.height < 2 || bottom.height < 2 {
            if self.split > 0.5 {
                self.children[0].draw(f, area);
            } else {
                self.children[1].draw(f, area);
            }
        } else {
            self.children[0].draw(f, top);
            self.children[1].draw(f, bottom);
        }
    }

    fn set_style(&mut self, style: WStyleOpt) {
        self.children[0].set_style(style.clone());
        self.children[1].set_style(style);
    }

    fn set_override_style(&mut self, style: WStyleOpt) {
        self.children[0].set_override_style(style.clone());
        self.children[1].set_override_style(style);
    }

    fn unset_override_style(&mut self) {
        self.children[0].unset_override_style();
        self.children[1].unset_override_style();
    }

    fn set_child(&mut self, index: usize, child: Box<dyn Container>) {
        if index < self.children.len() {
            self.children[index] = child;
        }
    }

    fn get_child(&self, index: u8) -> Option<&Box<dyn Container>> {
        if index < self.children.len() as u8 {
            Some(&self.children[index as usize])
        } else {
            None
        }
    }

    fn get_child_mut(&mut self, index: u8) -> Option<&mut Box<dyn Container>> {
        if index < self.children.len() as u8 {
            Some(&mut self.children[index as usize])
        } else {
            None
        }
    }

    fn set_widget(&mut self, widget: Box<dyn Widget>) {
    }

    fn get_widget(&self) -> Option<&Box<dyn Widget>> {
        None
    }

    fn get_widget_mut(&mut self) -> Option<&mut Box<dyn Widget>> {
        None
    }

    fn has_children(&self) -> bool {
        true
    }
}

impl Default for VSplitContainer {
    fn default() -> Self {
        VSplitContainer {
            children: vec![Box::new(BasicContainer::default()), Box::new(BasicContainer::default())],
            split: 0.5,
        }
    }
}

impl VSplitContainer {
    pub fn new(top: Box<dyn Container>, bottom: Box<dyn Container>, split: f32) -> Self {
        VSplitContainer {
            children: vec![top, bottom],
            split,
        }
    }
}


pub struct BasicWidget {
    title: String,
    text: String,
    style: WStyle,
    override_style: Option<WStyleOpt>,
}

impl Widget for BasicWidget {
    fn draw(&self, f: &mut Frame<F>, area: Rect) {
        let local_style = self.get_style();
        let rect = area;
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(local_style.border_style)
            .title(Span::styled(self.title.clone(), local_style.title_style));
        f.render_widget(block, rect);
        let text = Text::styled(self.text.clone(), local_style.text_style);
        let text = Paragraph::new(text)
            .block(Block::default().borders(Borders::NONE))
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Left);
        f.render_widget(text, Rect::new(rect.x + 1, rect.y + 1, rect.width - 2, rect.height - 2));
    }

    fn get_style(&self) -> WStyle {
        match &self.override_style {
            Some(style) => self.style.clone().set(style.clone()).to_owned(),
            None => self.style.clone(),
        }
    }

    fn set_override_style(&mut self, style: WStyleOpt) {
        self.override_style = Some(style);
    }

    fn unset_override_style(&mut self) {
        self.override_style = None;
    }

    fn set_style(&mut self, style: WStyleOpt) {
        self.style.set(style);
    }
}

impl Default for BasicWidget {
    fn default() -> Self {
        BasicWidget {
            title: String::from(""),
            text: String::from(""),
            style: WStyle::default(),
            override_style: None
        }
    }
}

impl BasicWidget {
    pub fn new(title: String, text: String) -> Self {
        BasicWidget {
            title,
            text,
            style: WStyle::default(),
            override_style: None
        }
    }
}