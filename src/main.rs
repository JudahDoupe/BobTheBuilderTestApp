//! Button Clicker — a number you increase by clicking a button (issue #9 added the UI).
//!
//! Renders an on-screen counter with Increment / Reset buttons via `DefaultPlugins`.
//! The pure counting logic still lives in `counter` so it stays unit-testable without
//! spinning up a window.

mod counter;
mod ui;

use bevy::prelude::*;

// Re-export counter symbols for use in main() and tests.
pub(crate) use counter::{increment, ClickCount, ClickEvent};

/// Returns the primary window configuration for automatic device resolution scaling.
fn make_primary_window() -> Option<Window> {
    Some(Window {
        resizable: true,
        fit_canvas_to_parent: true,
        ..default()
    })
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: make_primary_window(),
                ..default()
            }),
        )
        .init_resource::<ClickCount>()
        .add_event::<ClickEvent>()
        .add_systems(Startup, ui::spawn_ui)
        .add_systems(
            Update,
            (
                ui::handle_buttons,
                counter::count_clicks,
                counter::reset_on_r,
                ui::update_count_text,
            ),
        )
        .run();
}

#[cfg(test)]
mod tests {
    use crate::increment;

    #[test]
    fn increment_adds_one() {
        assert_eq!(increment(0), 1);
        assert_eq!(increment(41), 42);
    }
}
