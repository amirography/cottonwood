use core::fmt;
use std::time::Duration;

use super::{general::GeneralWidgetProperties, Transition, Widget};

#[derive(Default, Clone)]
pub struct Revealer {
    general: Option<GeneralWidgetProperties>,
    transition: Option<Transition>,
    reveal: Option<bool>,
    duration: Option<Duration>,
    children: Option<Vec<Widget>>,
}

impl Revealer {
    pub fn new() -> Self {
        Self {
            general: None,
            transition: None,
            reveal: None,
            duration: None,
            children: None,
        }
    }

    pub fn set_general(&mut self, general: Option<GeneralWidgetProperties>) -> &mut Self {
        self.general = general;
        self
    }

    pub fn set_transition(&mut self, transition: Option<Transition>) -> &mut Self {
        self.transition = transition;
        self
    }

    pub fn set_reveal(&mut self, reveal: Option<bool>) -> &mut Self {
        self.reveal = reveal;
        self
    }

    pub fn set_duration(&mut self, duration: Option<Duration>) -> &mut Self {
        self.duration = duration;
        self
    }

    pub fn set_children(&mut self, children: Option<Vec<Widget>>) -> &mut Self {
        self.children = children;
        self
    }
}

impl fmt::Display for Revealer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = String::new();

        if let Some(reveal) = &self.reveal {
            buf = format!("{} :reveal {}", buf, reveal)
        };
        if let Some(duration) = &self.duration {
            buf = format!("{} :duration {}ms", buf, duration.subsec_millis())
        };
        if let Some(transition) = &self.transition {
            buf = format!("{} :transition {}", buf, transition)
        };

        if let Some(general) = &self.general {
            buf = format!("{} {}", buf, general)
        }

        if let Some(children) = &self.children {
            children.iter().for_each(|c| {
                buf = format!(
                    "{} {}",
                    buf,
                    match c {
                        Widget::Box(c) => format!("{}", c),
                        Widget::Label(c) => format!("{}", c),
                    }
                )
            })
        }

        write!(f, "(box {})", buf)
    }
}
