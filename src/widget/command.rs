use iced::{Column, Length, Point, Rectangle, Text};
use iced_aw::{TabBar, TabLabel};
use iced_native::layout::{Limits, Node};
use iced_native::{Clipboard, Event, Hasher, Layout, Widget};
use iced_native::event::Status;
use iced_native::overlay::Element;

#[derive(Clone, Hash)]
pub enum CmdParam {
    Text(String),
}

#[derive(Clone, Hash)]
pub enum Cmd {
    Text(String),
}

pub struct Command {
    params: Vec<CmdParam>,
    cmds: Vec<Cmd>,
}

impl Command {
    pub fn new() -> Self {
        Self {
            params,
            cmds,
        }
    }
}

impl <Message, Renderer> Widget<Message, Renderer> for Command where Renderer: render::Renderer,
{
    fn width(&self) -> Length {
        Length::Shrink
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
        let mut content = Column::new();
        for param in self.params.iter() {
            content = content.push(EditText)
        }
        todo!()
    }

    fn draw(&self, renderer: &mut Renderer, defaults: &iced_native::renderer::Defaults, layout: Layout<'_>, cursor_position: Point, viewport: &Rectangle) -> iced_native::renderer::Output {
        todo!()
    }

    fn hash_layout(&self, state: &mut Hasher) {
    }

}
