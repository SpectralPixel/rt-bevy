use bevy::prelude::*;
use crate::prelude::*;

const PLAYER_SPEED: f32 = 100.;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_initialize);
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
    //println!("Player position: {:?}", transform.translation);
}

pub fn player_point_at(target_position: Vec2, transform: &mut Transform, player_direction: &mut Direction) {
    let direction = target_position - transform.translation.truncate();
    let angle = direction.y.atan2(direction.x);
    transform.rotation = Quat::from_rotation_z(angle);
    player_direction.set(angle);
    // println!("Player rotation: {:?}", transform.rotation.to_axis_angle());
}
