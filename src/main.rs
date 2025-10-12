use iced::widget::{Space, button, column, text, text_input};
use iced::{Alignment, Element, Event, Length, Task as Command, Theme, event};

use iced_sessionlock::MultiApplication;
use iced_sessionlock::actions::UnLockAction;
use iced_sessionlock::settings::Settings;
use iced_sessionlock::to_session_message;

pub fn main() -> Result<(), iced_sessionlock::Error> {
  Counter::run(Settings::default())
}

struct Counter {
  value: std::sync::atomic::AtomicI32,
  text: String,
}

#[to_session_message]
#[derive(Debug, Clone)]
enum Message {
  IncrementPressed,
  DecrementPressed,
  TextInput(String),
  IcedEvent(Event),
  WindowOpened(iced::window::Id),
}

impl MultiApplication for Counter {
  type Message = Message;
  type Flags = ();
  type Theme = Theme;
  type Executor = iced::executor::Default;

  fn new(_flags: ()) -> (Self, Command<Message>) {
    (
      Self {
        value: 0.into(),
        text: "eee".to_string(),
      },
      Command::none(),
    )
  }

  fn namespace(&self) -> String {
    String::from("Counter - Iced")
  }

  fn subscription(&self) -> iced::Subscription<Self::Message> {
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
        println!("hello {event:?}");
        Command::none()
      }
      Message::IncrementPressed => {
        *self.value.get_mut() += 1;
        Command::none()
      }
      Message::DecrementPressed => {
        *self.value.get_mut() -= 1;
        Command::none()
      }
      Message::TextInput(text) => {
        self.text = text;
        Command::none()
      }
      Message::WindowOpened(Id) => Command::none(),
      Message::UnLock => Command::done(message),
    }
  }

  fn view(&self, _id: iced::window::Id) -> Element<Message> {
    column![
      Space::with_height(Length::Fill),
      button("Increment").on_press(Message::IncrementPressed),
      button("Lock").on_press(Message::UnLock),
      text(self.value.load(std::sync::atomic::Ordering::Relaxed)).size(50),
      text_input("hello", &self.text)
        .on_input(Message::TextInput)
        .padding(10),
      button("Decrement").on_press(Message::DecrementPressed),
      Space::with_height(Length::Fill),
    ]
    .padding(20)
    .align_x(Alignment::Center)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
  }
}
