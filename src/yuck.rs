pub fn wrap_in_label(class: Option<String>, text: String) -> String {
    let mut options = String::from("");

    match class {
        Some(class) => options = format!("{options} :class \"{}\"", class),
        None => (),
    }

    options = format!("{options} :text \"{}\"", text);

    format!("(label {})", options)
}

pub fn wrap_in_box(
    class: Option<String>,
    children: Option<String>,
    spacing: Option<i64>,
    orientation: Orientation,
    space_evenly: bool,
) -> String {
    let mut options = String::from("");

    match class {
        Some(class) => options = format!("{options} :class \"{}\"", class),
        None => (),
    }
    match spacing {
        Some(spacing) => options = format!("{options} :spacing {}", spacing),
        None => (),
    }

    options = format!("{options} :space-evenly {}", space_evenly);

    match orientation {
        Orientation::Vertical => options = format!("{options} :orientation \"v\""),
        Orientation::Horizontal => options = format!("{options} :orientation \"h\""),
    }

    match children {
        Some(class) => options = format!("{options}  {}", class),
        None => (),
    }

    format!("(box {})", options)
}

#[derive(Debug)]
pub enum Orientation {
    Horizontal,
    Vertical,
}
