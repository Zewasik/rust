use chrono::Duration;
use colored::*;
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub enum Position {
    Top,
    Bottom,
    Center,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Notification {
    size: u32,
    color: (u8, u8, u8),
    position: Position,
    content: String,
}

#[derive(Debug)]
pub enum Event<'a> {
    Remainder(&'a str),
    Registration(Duration),
    Appointment(&'a str),
    Holiday,
}

impl fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (r, g, b) = self.color;
        write!(
            f,
            "{:?}, {}, {}",
            self.position,
            self.size,
            self.content.truecolor(r, g, b)
        )
    }
}

impl Event<'_> {
    pub fn notify(&self) -> Notification {
        match self {
            Event::Remainder(value) => Notification {
                size: 50,
                color: (50, 50, 50),
                position: Position::Bottom,
                content: value.to_string(),
            },
            Event::Registration(dur) => Notification {
                size: 30,
                color: (255, 2, 22),
                position: Position::Top,
                content: format!(
                    "You have {}H:{}M:{}S left before the registration ends",
                    dur.num_hours(),
                    dur.num_minutes() % 60,
                    dur.num_seconds() % 60
                ),
            },
            Event::Appointment(value) => Notification {
                size: 100,
                color: (200, 200, 3),
                position: Position::Center,
                content: value.to_string(),
            },
            Event::Holiday => Notification {
                size: 25,
                color: (0, 255, 0),
                position: Position::Top,
                content: "Enjoy your holiday".to_string(),
            },
        }
    }
}
