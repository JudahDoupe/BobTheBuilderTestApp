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

fn count_clicks(mut clicks: EventReader<ClickEvent>, mut reset_events: EventReader<ResetEvent>, mut count: ResMut<ClickCount>) {
    for _ in clicks.read() {
        count.0 = increment(count.0);
    }
    for _ in reset_events.read() {
        count.0 = 0;
    }
}

fn increment(n: u32) -> u32 {
    n + 1
}

#[derive(Event)]
struct ResetEvent; // Define the ResetEvent struct

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .init_resource::<ClickCount>()
        .init_state::<GameState>()  // Change from insert_resource to init_resource for GameState
        .add_event::<ClickEvent>() // Register ClickEvent
        .add_event::<ResetEvent>() // Register ResetEvent
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
