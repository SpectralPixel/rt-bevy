use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera_setup);
    }
}

#[derive(Component)]
pub struct MainCamera;

pub fn camera_setup(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        Camera2d,
        Transform::from_translation(Vec3::ZERO),
    ));
}
