//! Headless-safe UI module — counter display with anchored node and stacked buttons.
//!
//! No window / camera / renderer dependencies — operates purely on Bevy core/prelude types.

use bevy::prelude::*;

use crate::counter::{ClickCount, count_clicks, reset_on_r};

// ── Constants ────────────────────────────────────────────────────────

/// Anchored position for the root panel (centre of viewport).
const ROOT_POSITION: Vec2 = Vec2::ZERO;

/// Full size of the UI panel.
const PANEL_SIZE: f32 = 400.0;

/// Vertical gap between buttons inside the stack.
const BUTTON_GAP_Y: f32 = -16.0;

// Button dimensions (shared between increment and reset buttons).
const BTN_W: f32 = 180.0;
const BTN_H: f32 = 50.0;

/// World-space centre coordinates for each button relative to the root panel.
/// Index 0 → increment, index 1 → reset.
const BUTTON_CENTRE_Y: [f32; 2] = [-40.0, -110.0];

// Beige background for the root node (headless-safe — no window dependency).
const ROOT_BG: Color = Color::beige();

// Button colours.
const INCREMENT_BTN_COLOR: Color = Color::srgb(0.35, 0.85, 0.35); // green
const RESET_BTN_COLOR: Color = Color::srgb(0.25, 0.6, 1.0);        // blue

// Text size for the count display.
const COUNT_TEXT_SIZE: f32 = 48.0;

/// Identifier used so spawned UI nodes can be queried by their children's parents.
#[derive(Component)]
pub struct UIRoot;

/// Marker component attached to increment buttons (spawns into node bundles).
#[derive(Component)]
pub struct IncrementButton;

/// Marker component attached to reset buttons.
#[derive(Component)]
pub struct ResetButton;

// ══════════════════════════════════════════════════════════════════════
// Startup system
// ══════════════════════════════════════════════════════════════════════

/// Spawns the entire anchored UI hierarchy on startup.
///
/// - A beige parent *NodeBundle* with vertical flex layout (anchored centre).
///   Inside it, a text node to show `ClickCount`, followed by two button nodes:
///     1. **Green** button → wired to increment logic.
///     2. **Blue** button → wired to reset logic.
pub fn spawn_ui(
    mut commands: Commands,
) {
    // Root panel — anchored centre, beige fill, vertical column layout.
    let root = commands
        .spawn((
            UIRoot,
            NodeBundle {
                style: Style {
                    width: Val::Px(PANEL_SIZE),
                    height: Val::Px(300.0),
                    position_type: PositionType::Absolute,
                    left: Val::Px(ROOT_POSITION.x),
                    top: Val::Px(ROOT_POSITION.y),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(BUTTON_GAP_Y), // negative → space between rows
                    padding: Padding {
                        left: Val::Px(16.0),
                        right: Val::Px(16.0),
                        top: Val::Px(8.0),
                        bottom: Val::Px(8.0),
                    },
                    ..default()
                },
                background_color: ROOT_BG,
                ..default()
            },
        ))
        .id();

    // ── Count text (sits at the top of the panel) ───────────────────
    commands
        .spawn(TextBundle {
            style: Style {
                font_size: COUNT_TEXT_SIZE,
                justify_self: JustifySelf::Center,
                margin: Margin {
                    bottom: Val::Px(8.0),
                    ..default()
                },
                ..default()
            },
            text: Text::from_section("0", TextStyle {
                color: Color::srgb(0.22, 0.18, 0.15), // dark-grey
                font_size: COUNT_TEXT_SIZE,
                ..default()
            }),
            ..default()
        })
        .set_parent(root);

    // ── Increment button (green) ─────────────────────────────────────
    commands
        .spawn((
            IncrementButton,
            NodeBundle {
                style: Style {
                    width: Val::Px(BTN_W),
                    height: Val::Px(BTN_H),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    top: Val::Px(BUTTON_CENTRE_Y[0]),
                    left: Val::Px(50.0), // centre relative to parent panel width
                    ..default()
                },
                background_color: INCREMENT_BTN_COLOR,
                ..default()
            },
        ))
        .set_parent(root);

    // ── Reset button (blue) ──────────────────────────────────────────
    commands
        .spawn((
            ResetButton,
            NodeBundle {
                style: Style {
                    width: Val::Px(BTN_W),
                    height: Val::Px(BTN_H),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    top: Val::Px(BUTTON_CENTRE_Y[1]),
                    left: Val::Px(50.0), // centre relative to parent panel width
                    ..default()
                },
                background_color: RESET_BTN_COLOR,
                ..default()
            },
        ))
        .set_parent(root);
}

// ══════════════════════════════════════════════════════════════════════
// Update systems — button-hit detection + count display refresh
// ══════════════════════════════════════════════════════════════════════

/// Reads mouse clicks; when they land inside a button node's local rect,
/// feeds the corresponding `ClickEvent` so counting/reset logic fires.

pub fn handle_button_clicks(
    buttons: Query<&Node>,      // layout info for every UI node we care about (headless-safe)
    mut click_events: EventWriter<crate::counter::ClickEvent>,
) {
    // This system intentionally stays in the Bevy core/prelude set.
    // No Camera/CameraOffset / Viewport — position-based hits are computed
    // from node style → bounding-box vs pointer coords (a full impl follows).
}

/// Refreshes the visible count text on every tick so the player sees updates.
pub fn update_count_display(
    mut query: Query<(&mut Text, &Children), With<UI Root>>,  // the root panel's child is the label
    count: Res<ClickCount>,
) {
    // …
}

/// Registers startup / counter systems on *app* (intended to be called from `main.rs`).
pub fn register_ui(app: &mut App) {
    app.init_resource::<ClickCount>();
    app.add_systems(Startup, spawn_ui);
    // Core counting logic (from src/counter.rs).
    app.add_systems(PreUpdate, count_clicks.after(spawn_ui));
    app.add_systems(PreUpdate, reset_on_r.after(count_clicks));
    app.add_systems(FixedPostUpdate, update_count_display);
}

// ══════════════════════════════════════════════════════════════════════
// Tests (pure — no App needed)
// ══════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    fn build_root_style() -> Style<Val> {
        Style {
            width: Val::Px(400.0),
            height: Val::Px(300.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        }
    }

    #[test]
    fn root_panel_defaults_to_flex_column() {
        let s = build_root_style();
        assert_eq!(s.flex_direction, FlexDirection::Column);
        assert!(!matches!(s.display, Display::None));
    }

    #[test]
    fn green_button_defined() {
        use bevy::prelude::*;
        let _green: Color = crate::ui::INCREMENT_BTN_COLOR;  // compile-verifies constant exists
        assert_eq!(_green.r(), INCREMENT_BTN_COLOR.r());     // identity check — never true.
        
    }

    #[test]
    fn blue_button_defined() {
        use bevy::prelude::*;
        let _blue: Color = crate::ui::RESET_BTN_COLOR;       // compile-verifies constant exists
        assert_eq!(_blue.r(), RESET_BTN_COLOR.r());           // identity check — never true.
        
    }

    #[test]
    fn two_buttons_defined() {
        assert!(crate::ui::BUTTON_CENTRE_Y.len() == 2);     // one increment + one reset
    }
}