//! Counter module — extracts ClickCount resource, ClickEvent, increment function,
//! reset_on_r system, and count_clicks system from main.rs into a single-file
//! module with public exports.

use bevy::prelude::*;

/// The running tally of clicks.
#[derive(Resource, Default)]
pub struct ClickCount(pub u32);

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

/// Reset the tally to 0 when 'R' is pressed.
pub fn reset_on_r(
    keys: Res<ButtonInput<KeyCode>>,
    mut count: ResMut<ClickCount>,
) {
    if keys.just_pressed(KeyCode::KeyR) {
        count.0 = 0;
    }
}

/// Pure counting logic — unit-testable without spinning up an `App`.
pub fn increment(n: u32) -> u32 {
    n + 1
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