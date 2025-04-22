use crate::prelude::*;
use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, input_players);
    }
}

pub fn input_players(
    mut player_transforms: Query<(&mut Transform, &mut Direction), With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    for (mut player_transform, mut player_direction) in player_transforms.iter_mut() {
        let mut velocity: Vec2 = Vec2::ZERO;
        if keys.pressed(KeyCode::KeyD) {
            velocity += Vec2::X;
        }
        if keys.pressed(KeyCode::KeyW) {
            velocity += Vec2::Y;
        }
        if keys.pressed(KeyCode::KeyA) {
            velocity += Vec2::NEG_X;
        }
        if keys.pressed(KeyCode::KeyS) {
            velocity += Vec2::NEG_Y;
        }
        player_translate(velocity.normalize_or_zero(), &mut player_transform, &time);

        let window = windows.single();
        let (camera, camera_transform) = camera.single();

        // Only run if the cursor is in the window
        if let Some(mouse_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
        {
            player_point_at(mouse_position, &mut player_transform, &mut player_direction);
        }
    }
}
