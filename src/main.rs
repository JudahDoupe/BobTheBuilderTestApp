//! Button Clicker — you click a button and a number goes up. That is the only feature.
//!
//! Kept deliberately tiny: it exists so the orchestrator's verification harness has a
//! real Bevy project to `cargo build` + `cargo test` against.

use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;

mod counter;

use counter::ClickCount;
use counter::ClickEvent;
use counter::count_clicks;
use counter::increment;

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
