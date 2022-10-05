use iced::{Application, Clipboard, Column, Command, Container, Element, executor, Length, Settings, Text, window};
use iced::image::viewer::Renderer;
use iced_aw::native::TabBar;
use iced_aw::TabLabel;

fn main() -> iced::Result {
    let settings = Settings {
        antialiasing: true,
        window: window::Settings {
            size: (1200, 800),
            min_size: None,
            max_size: None,
            resizable: true,
            decorations: true,
            transparent: false,
            always_on_top: false,
            icon: None,
        },
        ..Settings::default()
    };
    MainPage::run(settings)
}

struct MainPage {
    active_tab: usize,
    cmd_active_tab: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    TabSelected(usize),
    TabCmdSelected(usize),
}

impl Application for MainPage {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            MainPage {
                active_tab: 0,
                cmd_active_tab: 0,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Dev Tools")
    }

    fn update(&mut self, message: Message, clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::TabSelected(index) => {
                self.active_tab = index;
                Command::none()
            }
            Message::TabCmdSelected(index) => {
                // self.active_tab = index;
                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let mut content = Column::new();
        let title = Text::new("Dev Tools").size(50);
        content = content.push(title);
        let tab = TabBar::new(self.active_tab, Message::TabSelected)
            .push(TabLabel::Text(String::from("command")))
            .push(TabLabel::Text(String::from("base64")));
        content = content.push(tab);
        if self.active_tab == 0 {
            let cmd_title = Text::new("Command").size(30);
            content = content.push(cmd_title);
            let cmd_tab = TabBar::new(self.cmd_active_tab, Message::TabCmdSelected)
                .push(TabLabel::Text(String::from("kafka")))
                .push(TabLabel::Text(String::from("pulsar")));
            content = content.push(cmd_tab);
        }
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
