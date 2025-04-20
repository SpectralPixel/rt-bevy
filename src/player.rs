use std::f32::consts::PI;

use bevy::prelude::*;

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
        Sprite::from_image(asset_server.load("samplecircle16.png")),
    ));
}

pub fn player_translate(velocity: Vec2, transform: &mut Transform, time: &Res<Time>) {
    let delta_time = time.delta().as_secs_f32();
    transform.translation.x += velocity.x * PLAYER_SPEED * delta_time;
    transform.translation.y += velocity.y * PLAYER_SPEED * delta_time;
    //println!("Player position: {:?}", transform.translation);
}

pub fn player_point_at(target_position: Vec2, transform: &mut Transform) {
    let direction = target_position - transform.translation.truncate();
    let angle = direction.y.atan2(direction.x);
    transform.rotation = Quat::from_rotation_z(angle);
    //println!("Player rotation: {:?}", transform.rotation.to_axis_angle());
    println!("Player rotation: {:?}", calculate_angle(transform));
}

fn calculate_angle(transform: &Transform) -> f32 {
    let rot = transform.rotation.to_axis_angle();
    let p = rot.0.z;
    let q = rot.1;
    ((-p).clamp(0., 1.) * 2. * PI - q).abs()
}
