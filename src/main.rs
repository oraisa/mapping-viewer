mod bindings;
mod parser;
use std::path::PathBuf;
use structopt::StructOpt;

use iced::{Column, Row, Text, Application, executor,
           Command, Element, Scrollable, scrollable,
           Button, button, Settings, Length,
           VerticalAlignment, HorizontalAlignment, Space,
           Subscription, keyboard,
};
use iced_native::subscription;
use iced_native::event;

#[derive(StructOpt, Debug)]
#[structopt(name = "Mapping Viewer", about = "Shows keymap cheatsheets.")]
struct Opt {
    #[structopt(parse(from_os_str))]
    keymap: PathBuf
}

struct MappingViewer {
    window: bindings::Window,
    selected_tab: usize,
    scroll_state: scrollable::State,
    buttons: ButtonState
}

#[derive(Default)]
struct ButtonState {
    next_button: button::State,
    prev_button: button::State
}
impl ButtonState {
    fn view(&mut self, tab_name: &str) -> Row<Message> {
        Row::new()
            .push(Button::new(&mut self.prev_button, Text::new("<-")).on_press(Message::PrevTab))
            .push(
                Text::new(tab_name)
                    .vertical_alignment(VerticalAlignment::Center)
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .width(Length::Fill)
                    .size(22)
            )
            .push(Button::new(&mut self.next_button, Text::new("->")).on_press(Message::NextTab))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    NextTab,
    PrevTab,
    Exit
}

fn view_binding_group(group: &bindings::BindingGroup) -> Column<Message> {
    let binding_rows = group.bindings.iter().map(view_binding);
    let mut binding_col = Column::new().padding(5).spacing(3);
    for brow in binding_rows {
        binding_col = binding_col.push(brow);
    }
    Column::new().push(Text::new(&group.title).size(20)).push(binding_col)
}


fn view_binding(binding: &bindings::Binding) -> Row<Message> {
    const TEXT_SIZE: u16 = 17;
    Row::new()
        .push(Text::new(&binding.keys)
                .width(Length::FillPortion(2))
                .vertical_alignment(VerticalAlignment::Center)
                .horizontal_alignment(HorizontalAlignment::Left)
                .size(TEXT_SIZE)
        )
        .push(Space::new(Length::Units(20), Length::Shrink))
        .push(Text::new(&binding.action)
                .width(Length::FillPortion(3))
                .vertical_alignment(VerticalAlignment::Center)
                .horizontal_alignment(HorizontalAlignment::Left)
                .size(TEXT_SIZE)
        )
}

impl Application for MappingViewer {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn view(&mut self) -> Element<Message> {

        let tab_index = self.selected_tab % self.window.tabs.len();
        let tab_name = &self.window.tabs[tab_index].title;
        let tabline = self.buttons.view(tab_name);
        let rows = self.window.tabs[tab_index].groups
                       .iter()
                       .map(|binding_group| view_binding_group(&binding_group))
                       .collect::<Vec<_>>();
        let mut column = Column::new().spacing(10).padding(20);
        column = column.push(tabline);
        for row in rows {
            column = column.push(row);
        }
        Scrollable::new(&mut self.scroll_state).push(column).into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        self.selected_tab = match message {
            Message::NextTab => self.selected_tab.wrapping_add(1),
            Message::PrevTab => self.selected_tab.wrapping_sub(1),
            Message::Exit => std::process::exit(0) //TODO: cleaner way to exit
        };
        Command::none()
    }

    fn title(&self) -> String {
        String::from(&self.window.title)
    }

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let opt = Opt::from_args();
        let contents = std::fs::read_to_string(opt.keymap).expect("Unable to open file");
        (
            MappingViewer {
                window: parser::parse_bindings(&contents),
                scroll_state: scrollable::State::default(),
                selected_tab: 0,
                buttons: ButtonState::default(),
            },
            Command::none()
        )
    }

    fn subscription(&self) -> Subscription<Message> {
        subscription::events_with(|event, _status| {
            match event {
                event::Event::Keyboard(kevent) => match kevent  {
                    keyboard::Event::KeyPressed {key_code:code, modifiers: _modif} => match code {
                        keyboard::KeyCode::H => Some(Message::PrevTab),
                        keyboard::KeyCode::Left => Some(Message::PrevTab),
                        keyboard::KeyCode::L => Some(Message::NextTab),
                        keyboard::KeyCode::Right => Some(Message::NextTab),
                        keyboard::KeyCode::Escape => Some(Message::Exit),
                        _ => None
                    },
                    _ => None
                },
                _ => None
            }
        })
    }
}

fn main() -> iced::Result {
    MappingViewer::run(Settings {
        window: iced::window::Settings {
            size: (500, 800),
            resizable: false,
            always_on_top: true,
            ..iced::window::Settings::default()
        },
        ..iced::Settings::default()
    })
}
