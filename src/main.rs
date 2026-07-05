//! Button Clicker — you click a button and a number goes up. That is the only feature.
//!
//! Kept deliberately tiny: it exists so the orchestrator's verification harness has a
//! real Bevy project to `cargo build` + `cargo test` against.

mod counter;

use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;

// Re-export counter module symbols for use in main() and tests.
pub(crate) use counter::{increment, ClickCount, ClickEvent};

fn main() {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_once()))
        .init_resource::<ClickCount>()
        .add_event::<ClickEvent>()
        .add_systems(Update, (counter::count_clicks, counter::reset_on_r))
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
