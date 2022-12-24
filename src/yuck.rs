use strum_macros::Display;

use self::{boxes::Boxes, label::Label};
pub mod boxes;
pub mod general;
pub mod label;
pub mod revealer;

#[derive(Display, Debug, Clone)]
pub enum Orientation {
    #[strum(serialize = "\"h\"")]
    Horizontal,
    #[strum(serialize = "\"v\"")]
    Vertical,
}

#[derive(Display, Debug, Clone)]
pub enum Transition {
    #[strum(serialize = "\"slideright\"")]
    SlideRight,
    #[strum(serialize = "\"slideleft\"")]
    SlideLeft,
    #[strum(serialize = "\"slideup\"")]
    SlideUp,
    #[strum(serialize = "\"slidedown\"")]
    SlideDown,
    #[strum(serialize = "\"crossfade\"")]
    Crossfade,
    #[strum(serialize = "\"none\"")]
    None,
}

#[derive(Clone, Display, Debug)]
pub enum Widget {
    Box(Boxes),
    Label(Label),
}
