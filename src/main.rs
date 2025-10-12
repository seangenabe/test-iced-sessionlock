use iced::widget::{Space, button, column, text, text_input};
use iced::{Alignment, Element, Event, Length, Task as Command, Theme, event};

use iced_sessionlock::MultiApplication;
use iced_sessionlock::actions::UnLockAction;
use iced_sessionlock::settings::Settings;
use iced_sessionlock::to_session_message;

pub fn main() -> Result<(), iced_sessionlock::Error> {
  Counter::run(Settings::default())
}

#[derive(Default)]
struct WindowData {
  value: i32,
  text: String,
}

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

impl MultiApplication for Counter {
  type Message = Message;
  type Flags = ();
  type Theme = Theme;
  type Executor = iced::executor::Default;

  fn new(_flags: ()) -> (Self, Command<Message>) {
    (
      Self {
        state: std::collections::HashMap::new(),
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
        // println!("hello {event:?}");
        Command::none()
      }
      Message::IncrementPressed(id) => {
        if let Some(w) = self.state.get_mut(&id) {
          w.value += 1;
        }
        Command::none()
      }
      Message::DecrementPressed(id) => {
        if let Some(w) = self.state.get_mut(&id) {
          w.value -= 1;
        }
        Command::none()
      }
      Message::TextInput((id, text)) => {
        self.state.get_mut(&id).unwrap().text = text.clone();
        Command::none()
      }
      Message::WindowOpened(id) => {
        println!("opened window id {:?}", id);
        self.state.insert(id, WindowData::default());
        Command::none()
      }
      Message::UnLock => Command::done(message),
    }
  }

  fn view(&self, id: iced::window::Id) -> Element<'_, Message> {
    let id2 = id.clone();
    println!("view on window id {:?}", id2);
    column![
      Space::with_height(Length::Fill),
      button("Increment").on_press(Message::IncrementPressed(id)),
      button("Lock").on_press(Message::UnLock),
      text(
        self
          .state
          .get(&id)
          .map_or("loading...".to_string(), |d| d.value.to_string())
      )
      .size(50),
      text_input(
        "hello",
        &self.state.get(&id).map_or("loading...", |d| &d.text)
      )
      .on_input(move |text| Message::TextInput((id2, text.clone()))),
      text(format!(
        "text is {}",
        self.state.get(&id).map_or("loading...", |d| &d.text)
      )),
      button("Decrement").on_press(Message::DecrementPressed(id)),
      Space::with_height(Length::Fill),
    ]
    .padding(20)
    .align_x(Alignment::Center)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
  }
}
