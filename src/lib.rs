use bevy::prelude::*;
use crate::prelude::*;

pub mod prelude;

mod camera;
mod direction;
mod input;
mod player;
mod ray;
mod viewport;
mod grid;

pub struct SetupGamePlugin;

impl Plugin for SetupGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            CameraPlugin,
            PlayerPlugin,
            InputPlugin,
            GridPlugin,
        ))
        .add_systems(Update, close_on_esc);
    }
}

fn close_on_esc(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (window, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::Escape) {
            commands.entity(window).despawn();
        }
    }
}
