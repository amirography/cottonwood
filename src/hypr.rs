#[allow(clippy::style)]
#[warn(clippy::double_neg)]
use std::panic;

use hyprland::{
    data::{self, Workspace},
    event_listener::EventListener,
    shared::WorkspaceType,
};

use crate::yuck::{self, general::GeneralWidgetProperties, Orientation, Widget};

pub async fn workspace_listen() {
    if let Ok(active) = data::blocking::get_active_workspace() {
        workspace_change_handler(&active.id);
    };
    panic::set_hook(Box::new(|_info| {
        // do nothing
    }));
    let h = panic::catch_unwind(|| {
        let mut event_listener = EventListener::new();
        event_listener.add_workspace_change_handler(|id| workspace_change_handler(&id));
        event_listener.start_listener_blocking()
    });

    match h {
        Ok(_) => (),
        Err(_) => (),
    };
}

fn workspace_change_handler(id: &WorkspaceType) {
    if let Ok(mut ws) = data::blocking::get_workspaces() {
        ws.sort_by(|a, b| unwrap_workspace_type(&a.id).cmp(&unwrap_workspace_type(&b.id)));

        let mut childrens: Vec<Widget> = vec![];
        for w in ws {
            let a: Widget = format_workspace(
                &w,
                match &w.id == id {
                    true => String::from("active-workspace"),
                    false => match w.windows {
                        0 => String::from("inactive-workspace"),
                        _ => String::from("occupied-workspace"),
                    },
                },
            );

            childrens.push(a);
        }

        println!(
            "{}",
            yuck::boxes::Boxes::new()
                .set_general(Some(
                    GeneralWidgetProperties::new()
                        .set_class(Some(String::from("workspaces")))
                        .to_owned()
                ))
                .set_children(Some(childrens))
                .set_orientation(Some(Orientation::Horizontal))
                .set_space_evenly(Some(false))
        );
    };
}

fn unwrap_workspace_type(w: &WorkspaceType) -> u8 {
    match w {
        WorkspaceType::Regular(i) => i.to_owned(),
        WorkspaceType::Special => 0,
    }
}

fn format_workspace(workspace: &Workspace, class: String) -> Widget {
    let n = yuck::label::Label::new(&format!("{}", workspace.name))
        .set_general(Some(
            GeneralWidgetProperties::new()
                .set_class(Some(class.to_owned()))
                .set_tooltip(Some(format!(
                    "{}",
                    get_client(unwrap_workspace_type(&workspace.id))
                )))
                .to_owned(),
        ))
        .to_owned();
    // let w = yuck::label::Label::new(&format!(
    //     "{}",
    //     get_client(unwrap_workspace_type(&workspace.id))
    // ))
    // .set_general(Some(
    //     GeneralWidgetProperties::new()
    //         .set_class(Some(class))
    //         .set_tooltip(Some(format!(
    //             "{}",
    //             get_client(unwrap_workspace_type(&workspace.id))
    //         )))
    //         .to_owned(),
    // ))
    // .to_owned();

    yuck::Widget::Box(
        yuck::boxes::Boxes::new()
            .set_children(Some(vec![Widget::Label(n)]))
            .to_owned(),
    )
}

fn get_client(w: u8) -> String {
    let mut v: String = String::new();
    match hyprland::data::blocking::get_clients() {
        Ok(c) => c
            .iter()
            .filter(|cl| unwrap_workspace_type(&cl.workspace.id) == w)
            // .for_each(|cl| v.push(String::from(cl.class.split('.').last().unwrap()))),
            .for_each(|cl| {
                v = format!(
                    "{}   {}-{}",
                    v,
                    match cl.class.split('.').last().unwrap() {
                        "wezterm" => "",
                        "kitty" => "",
                        "firefox" => "",
                        "eww" => "",
                        "desktop" => "",
                        _ => " ",
                    },
                    String::from(
                        &cl.title
                            .replace(" - ", "-")
                            .replace(" ", "_")
                            .to_lowercase()
                    )
                )
            }),
        Err(_) => return v,
    };
    return v;
}
