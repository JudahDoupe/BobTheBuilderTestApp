//! Viewport-constrained sizing for the gameplay area.

use bevy::prelude::*;

/// Marker component for the gameplay area node.
#[derive(Component)]
pub struct GameplayArea {}

/// Resizes the gameplay area to `min(viewport_width, viewport_height)` pixels on both axes.
pub fn fnsized_gameplay_area(
    mut query: Query<&mut Node, With<GameplayArea>>,
    primary_window: Query<&'static Window>,
) {
    let window = primary_window.single();
    let size = window.width().min(window.height());

    for mut node in &mut query {
        node.width = Val::Px(size);
        node.height = Val::Px(size);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn GameplayArea_exists() {
        let _marker: GameplayArea = GameplayArea {};
    }
}