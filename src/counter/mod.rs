//! Counter module — extracted types and systems from `main.rs`.

use bevy::prelude::*;

/// The running tally of clicks.
#[derive(Resource, Default)]
pub struct ClickCount(u32);

/// Fired when the player clicks the button.
#[derive(Event)]
pub struct ClickEvent;

/// Advance the tally by one for every click received this frame.
pub fn count_clicks(
    mut clicks: EventReader<ClickEvent>,
    mut count: ResMut<ClickCount>,
) {
    for _ in clicks.read() {
        count.0 = increment(count.0);
    }
}

/// Pure counting logic — unit-testable without spinning up an `App`.
pub fn increment(n: u32) -> u32 {
    n + 1
}