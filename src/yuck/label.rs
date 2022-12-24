use core::fmt;

use super::general::GeneralWidgetProperties;

#[derive(Default, Clone, Debug)]
pub struct Label {
    general: Option<GeneralWidgetProperties>,
    text: String,
    limit_width: Option<i64>,
    show_truncated: Option<bool>,
    markup: Option<String>,
    wrap: Option<bool>,
    angle: Option<u32>,
    xalign: Option<u32>,
    yalign: Option<u32>,
}

impl Label {
    pub fn new(text: &str) -> Self {
        Self {
            general: None,
            text: String::from(text),
            limit_width: None,
            show_truncated: None,
            markup: None,
            wrap: None,
            angle: None,
            xalign: None,
            yalign: None,
        }
    }
    pub fn set_general(&mut self, general: Option<GeneralWidgetProperties>) -> &mut Self {
        self.general = general;
        self
    }

    pub fn set_text(&mut self, text: String) -> &mut Self {
        self.text = text;
        self
    }

    pub fn set_limit_width(&mut self, limit_width: Option<i64>) -> &mut Self {
        self.limit_width = limit_width;
        self
    }

    pub fn set_show_truncated(&mut self, show_truncated: Option<bool>) -> &mut Self {
        self.show_truncated = show_truncated;
        self
    }

    pub fn set_markup(&mut self, markup: Option<String>) -> &mut Self {
        self.markup = markup;
        self
    }

    pub fn set_wrap(&mut self, wrap: Option<bool>) -> &mut Self {
        self.wrap = wrap;
        self
    }

    pub fn set_angle(&mut self, angle: Option<u32>) -> &mut Self {
        self.angle = angle;
        self
    }

    pub fn set_xalign(&mut self, xalign: Option<u32>) -> &mut Self {
        self.xalign = xalign;
        self
    }

    pub fn set_yalign(&mut self, yalign: Option<u32>) -> &mut Self {
        self.yalign = yalign;
        self
    }
}

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = String::new();

        if let Some(limit_width) = &self.limit_width {
            buf = format!("{} :limit-width {}", buf, limit_width)
        }
        if let Some(show_truncated) = &self.show_truncated {
            buf = format!("{} :show_truncated {}", buf, show_truncated)
        }
        if let Some(markup) = &self.markup {
            buf = format!("{} :markup \"{}\"", buf, markup)
        }
        if let Some(wrap) = &self.wrap {
            buf = format!("{} :wrap {}", buf, wrap)
        }
        if let Some(angle) = &self.angle {
            buf = format!("{} :angle {}", buf, angle)
        }
        if let Some(xalign) = &self.xalign {
            buf = format!("{} :xalign {}", buf, xalign)
        }
        if let Some(yalign) = &self.yalign {
            buf = format!("{} :yalign {}", buf, yalign)
        }
        if let Some(general) = &self.general {
            buf = format!("{} {}", buf, general)
        }

        buf = format!("{} :text \"{}\"", buf, &self.text);

        write!(f, "(label {})", buf)
    }
}
