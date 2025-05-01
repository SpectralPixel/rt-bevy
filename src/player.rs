use crate::prelude::*;
use bevy::prelude::*;

const PLAYER_SPEED: f32 = 100.;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize);
        app.add_systems(Update, (viewport_gizmo, grid_pos));
    }
}

#[derive(Component)]
pub struct Player;

fn initialize(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player,
        Direction::new(0.),
        RayViewport2D::new(Vec2::ZERO, Direction::new(0.), 90., 7., 25),
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

fn viewport_gizmo(
    mut gizmos: Gizmos,
    player_viewport: Query<&RayViewport2D, With<Player>>,
    grid: Query<&Grid2D>,
) {
    let grid = grid.single();
    for viewport in player_viewport.iter() {
        viewport.cast_rays(&mut gizmos, &grid);
    }
}

fn grid_pos(player_transforms: Query<&Transform, With<Player>>, grid: Query<&Grid2D>) {
    let grid = grid.single();
    for player_transform in player_transforms.iter() {
        let player_pos = grid
            .to_grid_pos(player_transform.translation.truncate())
            .unwrap()
            .floor();
        println!(
            "Player position: {:?} ; {}",
            player_pos,
            grid.get(&player_pos).unwrap_or_default()
        );
    }
}
