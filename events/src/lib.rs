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
    pub size: u32,
    pub color: (u8, u8, u8),
    pub position: Position,
    pub content: String,
}

#[derive(Debug)]
pub enum Event<'a> {
    Remainder(&'a str),
    Registration(Duration),
    Appointment(&'a str),
    Holiday,
}

impl Event<'_> {
    pub fn notify(&self) -> Notification {
        match self {
            Event::Remainder(text) => Notification {
                size: 50,
                color: (50, 50, 50),
                position: Position::Bottom,
                content: text.to_string(),
            },
            Event::Registration(duration) => {
                let hours = duration.num_hours();
                let minutes = (duration.num_minutes() % 60) as u32;
                let seconds = (duration.num_seconds() % 60) as u32;
                Notification {
                    size: 30,
                    color: (255, 2, 22),
                    position: Position::Top,
                    content: format!("You have {}H:{}M:{}S left before the registration ends", hours, minutes, seconds),
                }
            }
            Event::Appointment(text) => Notification {
                size: 100,
                color: (200, 200, 3),
                position: Position::Center,
                content: text.to_string(),
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

impl fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (r, g, b) = self.color;
        write!(
            f,
            "({:?}, {}, {})",
            self.position,
            self.size,
            self.content.truecolor(r, g, b)
        )
    }
}

// Exemple d'utilisation
fn main() {
    use Event::*;

    let remainder = Remainder("Go to the doctor");
    println!("{}", remainder.notify());
    let registration = Registration(Duration::seconds(49094));
    println!("{}", registration.notify());
    let appointment = Appointment("Go to the doctor");
    println!("{}", appointment.notify());
    let holiday = Holiday;
    println!("{}", holiday.notify());
}
