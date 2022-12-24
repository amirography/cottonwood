use std::fmt;

use strum_macros::Display;
#[derive(Clone, Debug)]
pub struct GeneralWidgetProperties {
    class: Option<String>,
    valign: Option<Align>,
    halign: Option<Align>,
    vexpand: Option<bool>,
    hexpand: Option<bool>,
    width: Option<i64>,
    height: Option<i64>,
    active: Option<bool>,
    tooltip: Option<String>,
    visible: Option<bool>,
    style: Option<String>,
}

impl fmt::Display for GeneralWidgetProperties {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = String::new();

        if let Some(class) = &self.class {
            buf = format!("{} :class \"{}\"", buf, class)
        }
        if let Some(valign) = &self.valign {
            buf = format!("{} :valign {}", buf, valign)
        }
        if let Some(halign) = &self.halign {
            buf = format!("{} :halign {}", buf, halign)
        }
        if let Some(hexpand) = &self.hexpand {
            buf = format!("{} :hexpand {}", buf, hexpand)
        }
        if let Some(vexpand) = &self.vexpand {
            buf = format!("{} :vexpand {}", buf, vexpand)
        }
        if let Some(width) = &self.width {
            buf = format!("{} :width {}", buf, width)
        }
        if let Some(height) = &self.height {
            buf = format!("{} :height {}", buf, height)
        }
        if let Some(active) = &self.active {
            buf = format!("{} :active {}", buf, active)
        }
        if let Some(tooltip) = &self.tooltip {
            buf = format!("{} :tooltip \"{}\"", buf, tooltip)
        }
        if let Some(visible) = &self.visible {
            buf = format!("{} :visible {}", buf, visible)
        }
        if let Some(style) = &self.style {
            buf = format!("{} :style \"{}\"", buf, style)
        }

        write!(f, "{}", buf)
    }
}

impl GeneralWidgetProperties {
    pub fn new() -> Self {
        Self {
            class: None,
            valign: None,
            halign: None,
            vexpand: None,
            hexpand: None,
            width: None,
            height: None,
            active: None,
            tooltip: None,
            visible: None,
            style: None,
        }
    }

    pub fn set_class(&mut self, class: Option<String>) -> &mut Self {
        self.class = class;
        self
    }

    pub fn set_valign(&mut self, valign: Option<Align>) -> &mut Self {
        self.valign = valign;
        self
    }

    pub fn set_halign(&mut self, halign: Option<Align>) -> &mut Self {
        self.halign = halign;
        self
    }

    pub fn set_vexpand(&mut self, vexpand: Option<bool>) -> &mut Self {
        self.vexpand = vexpand;
        self
    }

    pub fn set_hexpand(&mut self, hexpand: Option<bool>) -> &mut Self {
        self.hexpand = hexpand;
        self
    }

    pub fn set_width(&mut self, width: Option<i64>) -> &mut Self {
        self.width = width;
        self
    }

    pub fn set_height(&mut self, height: Option<i64>) -> &mut Self {
        self.height = height;
        self
    }

    pub fn set_active(&mut self, active: Option<bool>) -> &mut Self {
        self.active = active;
        self
    }

    pub fn set_tooltip(&mut self, tooltip: Option<String>) -> &mut Self {
        self.tooltip = tooltip;
        self
    }

    pub fn set_visible(&mut self, visible: Option<bool>) -> &mut Self {
        self.visible = visible;
        self
    }

    pub fn set_style(&mut self, style: Option<String>) -> &mut Self {
        self.style = style;
        self
    }
}

#[derive(Display, Debug, Clone)]
pub enum Align {
    #[strum(serialize = "\"fill\"")]
    Fill,
    #[strum(serialize = "\"baseline\"")]
    Baseline,
    #[strum(serialize = "\"center\"")]
    Center,
    #[strum(serialize = "\"start\"")]
    Start,
    #[strum(serialize = "\"end\"")]
    End,
}

#[cfg(test)]
mod tests {
    #[test]
    fn general_empty() {
        assert_eq!(format!("{}", super::GeneralWidgetProperties::new()), "");
    }

    #[test]
    fn general_with_class() {
        assert_eq!(
            format!(
                "{}",
                super::GeneralWidgetProperties::new().set_class(Some(String::from("classterm")))
            ),
            " :class \"classterm\""
        );
    }

    #[test]
    fn general_with_class_and_one_detail() {
        assert_eq!(
            format!(
                "{}",
                super::GeneralWidgetProperties::new()
                    .set_valign(Some(crate::yuck::general::Align::Fill))
                    .set_class(Some(String::from("classterm")))
            ),
            " :class \"classterm\" :valign \"fill\""
        );
    }

    // #[test]
    // fn general_with_all_arguments() {
    //     assert_eq!(
    //         format!(
    //             "{}",

    //         ),
    //         " :class \"classterm\" :valign \"fill\" :halign \"baseline\" :hexpand false :vexpand true :width 10 :height 190 :active false :tooltip \"something\" :visible true :style \"styleized\""
    //     );
    // }
}
