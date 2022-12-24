use std::fmt;

use super::{general::GeneralWidgetProperties, Orientation, Widget};

#[derive(Default, Clone, Debug)]
pub struct Boxes {
    general: Option<GeneralWidgetProperties>,
    spacing: Option<i64>,
    orientation: Option<Orientation>,
    space_evenly: Option<bool>,
    children: Option<Vec<Widget>>,
}

impl Boxes {
    pub fn new() -> Self {
        Self {
            general: None,
            spacing: None,
            orientation: None,
            space_evenly: None,
            children: None,
        }
    }

    pub fn set_general(&mut self, general: Option<GeneralWidgetProperties>) -> &mut Self {
        self.general = general;
        self
    }

    pub fn set_spacing(&mut self, spacing: Option<i64>) -> &mut Self {
        self.spacing = spacing;
        self
    }

    pub fn set_orientation(&mut self, orientation: Option<Orientation>) -> &mut Self {
        self.orientation = orientation;
        self
    }

    pub fn set_space_evenly(&mut self, space_evenly: Option<bool>) -> &mut Self {
        self.space_evenly = space_evenly;
        self
    }

    pub fn set_children(&mut self, children: Option<Vec<Widget>>) -> &mut Self {
        self.children = children;
        self
    }
}

impl fmt::Display for Boxes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = String::new();

        if let Some(spacing) = &self.spacing {
            buf = format!("{} :spacing {}", buf, spacing)
        }

        if let Some(orientation) = &self.orientation {
            buf = format!("{} :orientation {}", buf, orientation)
        }

        if let Some(space_evenly) = &self.space_evenly {
            buf = format!("{} :space-evenly {}", buf, space_evenly)
        }

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
