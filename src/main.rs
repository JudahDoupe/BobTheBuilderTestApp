//! Button Clicker — you click a button and a number goes up. That is the only feature.
//!
//! Kept deliberately tiny: it exists so the orchestrator's verification harness has a
//! real Bevy project to `cargo build` + `cargo test` against.

use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;

/// The running tally of clicks.
#[derive(Resource, Default)]
struct ClickCount(u32);

/// Fired when the player clicks the button.
#[derive(Event)]
struct ClickEvent;

/// Advance the tally by one for every click received this frame.
fn count_clicks(mut clicks: EventReader<ClickEvent>, mut count: ResMut<ClickCount>) {
    for _ in clicks.read() {
        count.0 = increment(count.0);
    }
}

/// Pure counting logic — unit-testable without spinning up an `App`.
fn increment(n: u32) -> u32 {
    n + 1
}

fn main() {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_once()))
        .init_resource::<ClickCount>()
        .add_event::<ClickEvent>()
        .add_systems(Update, count_clicks)
        .run();
}

#[cfg(test)]
mod tests {
    use super::increment;

    #[test]
    fn increment_adds_one() {
        assert_eq!(increment(0), 1);
        assert_eq!(increment(41), 42);
    }
}
