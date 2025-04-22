use crate::prelude::*;
use bevy::prelude::*;

const PLAYER_SPEED: f32 = 100.;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_initialize);
        app.add_systems(Update, forward_gizmo);
    }
}

#[derive(Component)]
pub struct Player;

pub fn player_initialize(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player,
        Direction::new(0.),
        Sprite::from_image(asset_server.load("samplecircle16.png")),
    ));
}

pub fn player_translate(velocity: Vec2, transform: &mut Transform, time: &Res<Time>) {
    let delta_time = time.delta().as_secs_f32();
    transform.translation.x += velocity.x * PLAYER_SPEED * delta_time;
    transform.translation.y += velocity.y * PLAYER_SPEED * delta_time;
}

pub fn player_point_at(
    target_position: Vec2,
    transform: &mut Transform,
    player_direction: &mut Direction,
) {
    let direction = target_position - transform.translation.truncate();
    let angle = direction.y.atan2(direction.x);
    transform.rotation = Quat::from_rotation_z(angle);
    player_direction.set(angle);
}

fn forward_gizmo(
    mut gizmos: Gizmos,
    player_transforms: Query<(&Transform, &Direction), With<Player>>,
) {
    for (transform, direction) in player_transforms.iter() {
        let forward = Vec2::from_angle(direction.get());
        let cur_pos = transform.translation.truncate();
        let end_pos = cur_pos + forward * 50.0;
        let color_red = Color::linear_rgb(1., 0., 0.);
        gizmos.arrow_2d(cur_pos, end_pos, color_red);
    }
}
