use bevy::prelude::*;

#[derive(Resource, Default)]
struct ClickCount(u32);

#[derive(Event)]
struct ClickEvent;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    #[default]
    Running,
    Paused,
}

fn count_clicks(mut clicks: EventReader<ClickEvent>, mut count: ResMut<ClickCount>) {
    for _ in clicks.read() {
        count.0 = increment(count.0);
    }
}

fn reset_count(mut reset_events: EventReader<ResetEvent>, mut count: ResMut<ClickCount>) {
    for _ in reset_events.read() {
        count.0 = 0;
    }
}

fn increment(n: u32) -> u32 {
    n + 1
}

#[derive(Event)]
struct ResetEvent;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .insert_resource(ClickCount(0))
        .init_state::<GameState>() // Change from init_resource to init_resource for GameState
        .add_event::<ClickEvent>() // Register ClickEvent
        .add_event::<ResetEvent>() // Register ResetEvent
        .add_systems(
            Update,
            (count_clicks, reset_count).run_if(in_state(GameState::Running))
        )
        .run();
}

#[cfg(test)]
mod tests {
    use super::{increment, ClickCount};

    #[test]
    fn increment_adds_one() {
        assert_eq!(increment(0), 1);
        assert_eq!(increment(41), 42);
    }
}
