use iced::widget::{button, column, text, text_input};
use iced::{Alignment, Element, Event, Length, Task as Command, event};

use iced_sessionlock::application;
use iced_sessionlock::to_session_message;

pub fn main() -> Result<(), iced_sessionlock::Error> {
  application(Counter::new, Counter::update, Counter::view)
    .subscription(Counter::subscription)
    .run()
}

#[derive(Default, Debug)]
struct WindowData {
  value: i32,
  text: String,
}

#[derive(Debug)]
struct Counter {
  state: std::collections::HashMap<iced::window::Id, WindowData>,
}

#[to_session_message]
#[derive(Debug, Clone)]
enum Message {
  IncrementPressed(iced::window::Id),
  DecrementPressed(iced::window::Id),
  TextInput((iced::window::Id, String)),
  IcedEvent(Event),
  WindowOpened(iced::window::Id),
}

impl Counter {
  fn new() -> (Self, Command<Message>) {
    (
      Self {
        state: std::collections::HashMap::new(),
      },
      Command::none(),
    )
  }

  fn subscription(&self) -> iced::Subscription<Message> {
    let window_open = event::listen_with(|event, _status, id| match event {
      Event::Window(window_event) => match window_event {
        iced::window::Event::Opened {
          position: _,
          size: _,
        } => Some(Message::WindowOpened(id)),
        _ => None,
      },
      _ => None,
    });

    iced::Subscription::batch([
      window_open,
      event::listen().map(Message::IcedEvent),
    ])
  }

  fn update(&mut self, message: Message) -> Command<Message> {
    match message {
      Message::IcedEvent(event) => {
        // println!("hello {event:?}");
        Command::none()
      }
      Message::IncrementPressed(id) => {
        let w = self.state.entry(id).or_default();
        w.value += 1;
        Command::none()
      }
      Message::DecrementPressed(id) => {
        let w = self.state.entry(id).or_default();
        w.value -= 1;
        Command::none()
      }
      Message::TextInput((id, text)) => {
        let w = self.state.entry(id).or_default();
        w.text = text;
        Command::none()
      }
      Message::WindowOpened(id) => {
        let _ = self.state.entry(id).or_default();
        Command::none()
      }
      Message::UnLock => Command::done(message),
    }
  }

  fn view(&self, id: iced::window::Id) -> Element<'_, Message> {
    println!("view on window id {:?}", id);
    dbg!(&self.state);
    let text_input_string: &str = match self.state.get(&id) {
      Some(d) => &d.text,
      None => &"loading...".to_string(),
    };
    column![
      button("Increment").on_press(Message::IncrementPressed(id)),
      button("Lock").on_press(Message::UnLock),
      text(
        self
          .state
          .get(&id)
          .map_or("loading...".to_string(), |d| { d.value.to_string() })
      )
      .size(50),
      text_input("hello", &text_input_string)
        .on_input(move |text| Message::TextInput((id, text.clone()))),
      text(format!("text is {}", text_input_string)),
      button("Decrement").on_press(Message::DecrementPressed(id)),
    ]
    .padding(20)
    .align_x(Alignment::Center)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
  }
}
