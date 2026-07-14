//! On-screen counter UI (issue #9).
//!
//! Renders the running [`ClickCount`] front-and-centre with two buttons under it:
//! a green **Increment** and a blue **Reset**, on a beige background with a
//! dark-grey number. Bevy 0.15 UI: spawn `Node` + `BackgroundColor` + `Text`
//! component tuples (the old `NodeBundle`/`TextBundle`/`Style` are gone).

use bevy::prelude::*;

use crate::counter::{ClickCount, ClickEvent};

// ── Palette (issue #9) ────────────────────────────────────────────────
const BEIGE: Color = Color::srgb(0.96, 0.91, 0.78);
const DARK_GREY: Color = Color::srgb(0.20, 0.20, 0.20);
const GREEN: Color = Color::srgb(0.30, 0.69, 0.31);
const BLUE: Color = Color::srgb(0.13, 0.45, 0.85);

/// Marks the text node that shows the current count so the update system can find it.
#[derive(Component)]
pub struct CountText;

/// Marks the green increment button.
#[derive(Component)]
pub struct IncrementButton;

/// Marks the game interaction area.
#[derive(Component)]
pub struct GameplayArea;

/// Marks the blue reset button.
#[derive(Component)]
pub struct ResetButton;

/// Spawns the camera and the full UI tree on startup.
pub fn spawn_ui(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(24.0),
                ..default()
            },
            BackgroundColor(BEIGE),
        ))
        .with_children(|root| {
            root.spawn((
                Node {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                GameplayArea,
            )).with_children(|area| {
                // The number, front and centre.
                area.spawn((
                    Text::new("0"),
                    TextFont {
                        font_size: 96.0,
                        ..default()
                    },
                    TextColor(DARK_GREY),
                    CountText,
                ));
                // Increment (green) then Reset (blue), stacked underneath.
                spawn_button(area, "Increment", GREEN, IncrementButton);
                spawn_button(area, "Reset", BLUE, ResetButton);
            });
        });
}

/// Spawns a labelled button carrying `marker`, as a child of `parent`.
fn spawn_button(parent: &mut ChildBuilder, label: &str, color: Color, marker: impl Component) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(220.0),
                height: Val::Px(64.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(color),
            marker,
        ))
        .with_children(|button| {
            button.spawn((
                Text::new(label),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

/// Translates button presses into the existing counter pipeline: the increment button
/// fires a [`ClickEvent`] (consumed by `counter::count_clicks`, the single source of truth
/// for the tally), while the reset button zeroes [`ClickCount`] directly.
pub fn handle_buttons(
    increment: Query<&Interaction, (Changed<Interaction>, With<IncrementButton>)>,
    reset: Query<&Interaction, (Changed<Interaction>, With<ResetButton>)>,
    mut clicks: EventWriter<ClickEvent>,
    mut count: ResMut<ClickCount>,
) {
    for interaction in &increment {
        if *interaction == Interaction::Pressed {
            clicks.send(ClickEvent);
        }
    }
    for interaction in &reset {
        if *interaction == Interaction::Pressed {
            count.0 = 0;
        }
    }
}

/// Keeps the on-screen number in sync with [`ClickCount`] whenever it changes.
pub fn update_count_text(count: Res<ClickCount>, mut query: Query<&mut Text, With<CountText>>) {
    if !count.is_changed() {
        return;
    }
    for mut text in &mut query {
        text.0 = count.0.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // The palette is defined and distinct — a cheap guard that the constants exist
    // and were not collapsed to one colour by a bad edit.
    #[test]
    fn palette_colours_are_distinct() {
        let colours = [BEIGE, DARK_GREY, GREEN, BLUE];
        for (i, a) in colours.iter().enumerate() {
            for b in &colours[i + 1..] {
                assert_ne!(a.to_srgba(), b.to_srgba());
            }
        }
    }
}
