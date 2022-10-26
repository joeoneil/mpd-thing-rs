use std::io::Stdout;
use termion::input::MouseTerminal;
use termion::raw::RawTerminal;
use tui::backend::{Backend, TermionBackend};
use tui::Frame;
use tui::style::Style;
use tui::layout::{Alignment, Rect};
use tui::text::{Span, Text};
use tui::widgets::{Block, Borders, Paragraph};

type F = TermionBackend::<MouseTerminal<RawTerminal<Stdout>>>;

pub trait Container<B: Backend> {
    fn draw(&self, f: &mut Frame<B>);

    fn set_style(&mut self, style: WStyleOpt);

    fn set_bounds(&mut self, bounds: Rect);
}

pub trait MetaContainer<B: Backend> {
    fn set_child(&mut self, index: usize, child: Box<dyn Container<B>>);

    fn get_child(&self, index: usize) -> Option<&Box<dyn Container<B>>>;
}

pub trait Widget {
    fn draw(&self, f: &mut Frame<F>);

    fn get_style(&self) -> &Style;

    fn set_style(&mut self, style: WStyleOpt);

    fn set_bounds(&mut self, bounds: Rect);
}

pub struct WStyle {
    title_style: Style,
    text_style: Style,
    border_style: Style,
}

pub struct WStyleOpt {
    title_style: Option<Style>,
    text_style: Option<Style>,
    border_style: Option<Style>,
}

impl Clone for WStyle {
    fn clone(&self) -> Self {
        WStyle {
            title_style: self.title_style,
            text_style: self.text_style,
            border_style: self.border_style,
        }
    }
}

impl Default for WStyle {
    fn default() -> Self {
        WStyle {
            title_style: Style::default(),
            text_style: Style::default(),
            border_style: Style::default(),
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

impl Clone for WStyleOpt {
    fn clone(&self) -> Self {
        WStyleOpt {
            title_style: self.title_style,
            text_style: self.text_style,
            border_style: self.border_style,
        }
    }
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

impl WStyle {
    pub fn new(title_style: Style, text_style: Style, border_style: Style) -> Self {
        WStyle {
            title_style,
            text_style,
            border_style,
        }
    }

    pub fn set(&mut self, style: WStyleOpt) {
        if let Some(title_style) = style.title_style {
            self.title_style = title_style;
        }
        if let Some(text_style) = style.text_style {
            self.text_style = text_style;
        }
        if let Some(border_style) = style.border_style {
            self.border_style = border_style;
        }
    }
}


pub struct BasicContainer {
    bounds: Rect,
    child: Box<dyn Widget>,
}

impl Container<F> for BasicContainer {
    fn draw(&self, f: &mut Frame<F>) {
        self.child.draw(f);
    }

    fn set_style(&mut self, style: WStyleOpt) {
        self.child.set_style(style);
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
        self.child.set_bounds(bounds);
    }
}

impl Default for BasicContainer {
    fn default() -> Self {
        BasicContainer {
            bounds: Rect::default(),
            child: Box::new(BasicWidget::default()),
        }
    }
}

impl BasicContainer {
    pub fn new(child: Box<dyn Widget>) -> Self {
        BasicContainer {
            bounds: Rect::default(),
            child,
        }
    }

    pub fn set_child(&mut self, child: Box<dyn Widget>, index: usize) {
        match index {
            0 => self.child = child,
            _ => {}, // just ignore it.
        }
    }

    pub fn get_child(&self, index: usize) -> Option<&Box<dyn Widget>> {
        match index {
            0 => Some(&self.child),
            _ => None,
        }
    }
}


pub struct RootContainer {
    bounds: Rect,
    child: Vec<Box<dyn Container<F>>>,
}

impl Container<F> for RootContainer {
    fn draw(&self, f: &mut Frame<F>) {
        for child in &self.child {
            child.draw(f);
        }
    }

    fn set_style(&mut self, style: WStyleOpt) {
        for child in &mut self.child {
            child.set_style(style.clone());
        }
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
        for child in &mut self.child {
            child.set_bounds(bounds);
        }
    }
}

impl MetaContainer<F> for RootContainer {
    fn set_child(&mut self, index: usize, child: Box<dyn Container<F>>) {
        match index {
            0 => self.child.push(child),
            _ => {}, // just ignore it.
        }
    }

    fn get_child(&self, index: usize) -> Option<&Box<dyn Container<F>>> {
        match index {
            0 => Some(&self.child[0]),
            _ => None,
        }
    }
}

impl Default for RootContainer {
    fn default() -> Self {
        RootContainer {
            bounds: Rect::default(),
            child: vec![],
        }
    }
}

impl RootContainer {
    pub fn new() -> Self {
        RootContainer {
            bounds: Rect::default(),
            child: vec![],
        }
    }
}


pub struct HSplitContainer {
    bounds: Rect,
    children: Vec<Box<dyn Container<F>>>,
    split: f32,
}

impl Container<F> for HSplitContainer {
    fn draw(&self, f: &mut Frame<F>) {
        self.children[0].draw(f);
        self.children[1].draw(f);
    }

    fn set_style(&mut self, style: WStyleOpt) {
        self.children[0].set_style(style.clone());
        self.children[1].set_style(style);
    }

    fn set_bounds(&mut self, bounds: Rect) {
        let left = Rect {
            x: bounds.x,
            y: bounds.y,
            width: (bounds.width as f32 * self.split) as u16,
            height: bounds.height,
        };
        let right = Rect {
            x: bounds.x + left.width,
            y: bounds.y,
            width: bounds.width - left.width,
            height: bounds.height,
        };
        self.children[0].set_bounds(left);
        self.children[1].set_bounds(right);
        self.bounds = bounds;
    }
}

impl MetaContainer<F> for HSplitContainer {
    fn set_child(&mut self, index: usize, child: Box<dyn Container<F>>,) {
        match index {
            0 => self.children.push(child),
            _ => {}, // just ignore it.
        }
    }

    fn get_child(&self, index: usize) -> Option<&Box<dyn Container<F>>> {
        match index {
            0 => Some(&self.children[0]),
            1 => Some(&self.children[1]),
            _ => None,
        }
    }
}

impl Default for HSplitContainer {
    fn default() -> Self {
        HSplitContainer {
            bounds: Rect::default(),
            children: vec![Box::new(BasicContainer::default()), Box::new(BasicContainer::default())],
            split: 0.5,
        }
    }
}

impl HSplitContainer {
    pub fn new(left: Box<dyn Container<F>>, right: Box<dyn Container<F>>, split: f32) -> Self {
        HSplitContainer {
            bounds: Rect::default(),
            children: vec![left, right],
            split,
        }
    }
}


pub struct VSplitContainer {
    bounds: Rect,
    children: Vec<Box<dyn Container<F>>>,
    split: f32,
}

impl Container<F> for VSplitContainer {
    fn draw(&self, f: &mut Frame<F>) {
        self.children[0].draw(f);
        self.children[1].draw(f);
    }

    fn set_style(&mut self, style: WStyleOpt) {
        self.children[0].set_style(style.clone());
        self.children[1].set_style(style);
    }

    fn set_bounds(&mut self, bounds: Rect) {
        let top = Rect {
            x: bounds.x,
            y: bounds.y,
            width: bounds.width,
            height: (bounds.height as f32 * self.split) as u16,
        };
        let bottom = Rect {
            x: bounds.x,
            y: bounds.y + top.height,
            width: bounds.width,
            height: bounds.height - top.height,
        };
        self.children[0].set_bounds(top);
        self.children[1].set_bounds(bottom);
        self.bounds = bounds;
    }
}

impl MetaContainer<F> for VSplitContainer {
    fn set_child(&mut self, index: usize,  child: Box<dyn Container<F>>) {
        match index {
            0 => self.children.push(child),
            _ => {}, // just ignore it.
        }
    }

    fn get_child(&self, index: usize) -> Option<&Box<dyn Container<F>>> {
        match index {
            0 => Some(&self.children[0]),
            1 => Some(&self.children[1]),
            _ => None,
        }
    }
}

impl Default for VSplitContainer {
    fn default() -> Self {
        VSplitContainer {
            bounds: Rect::default(),
            children: vec![Box::new(BasicContainer::default()), Box::new(BasicContainer::default())],
            split: 0.5,
        }
    }
}

impl VSplitContainer {
    pub fn new(top: Box<dyn Container<F>>, bottom: Box<dyn Container<F>>, split: f32) -> Self {
        VSplitContainer {
            bounds: Rect::default(),
            children: vec![top, bottom],
            split,
        }
    }
}


pub struct BasicWidget {
    bounds: Rect,
    title: String,
    text: String,
    style: WStyle,
}

impl Widget for BasicWidget {
    fn draw(&self, f: &mut Frame<F>) {
        let rect = self.bounds;
        let block = tui::widgets::Block::default()
            .borders(tui::widgets::Borders::ALL)
            .border_style(self.style.border_style)
            .title(Span::styled(self.title.clone(), self.style.title_style));
        f.render_widget(block, rect);
        let text = Text::styled(self.text.clone(), self.style.text_style);
        let text = Paragraph::new(text)
            .block(Block::default().borders(Borders::NONE))
            .alignment(Alignment::Left);
        f.render_widget(text, Rect::new(rect.x + 1, rect.y + 1, rect.width - 2, rect.height - 2));
    }

    fn get_style(&self) -> &Style {
        &self.style.text_style
    }

    fn set_style(&mut self, style: WStyleOpt) {
        self.style.set(style);
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }
}

impl Default for BasicWidget {
    fn default() -> Self {
        BasicWidget {
            bounds: Rect::default(),
            title: String::from(""),
            text: String::from(""),
            style: WStyle::default(),
        }
    }
}

impl BasicWidget {
    pub fn new(title: String, text: String) -> Self {
        BasicWidget {
            bounds: Rect::default(),
            title,
            text,
            style: WStyle::default(),
        }
    }
}