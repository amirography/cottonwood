use std::panic;

use hyprland::{
    data::{self, Workspace},
    event_listener::EventListener,
    shared::WorkspaceType,
};

use crate::yuck;

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
    if let Ok(ws) = data::blocking::get_workspaces() {
        let mut occupieds: Vec<&Workspace> = ws.iter().filter(|w| w.windows != 0).collect();

        occupieds.sort_by(|a, b| unwrap_workspace_type(&a.id).cmp(&unwrap_workspace_type(&b.id)));

        let mut formed = String::new();
        for ws in occupieds {
            formed = format!(
                "{formed} {}",
                format_workspace(
                    &ws.id,
                    match &ws.id == id {
                        true => String::from("active-workspace"),
                        false => String::from("inactive-workspace"),
                    },
                )
            );
        }

        println!(
            "{}",
            yuck::wrap_in_box(
                Some(String::from("workspaces")),
                Some(formed),
                None,
                yuck::Orientation::Horizontal,
                true,
            )
        );
    };
}

fn unwrap_workspace_type(w: &WorkspaceType) -> u8 {
    match w {
        WorkspaceType::Regular(i) => i.to_owned(),
        WorkspaceType::Special => 0,
    }
}

fn format_workspace(id: &WorkspaceType, class: String) -> String {
    format!(
        "{}",
        yuck::wrap_in_label(Some(class), format!("{}", unwrap_workspace_type(id)))
    )
}
