use bevy::prelude::*;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize);
        //app.add_systems(Update, viewport_gizmo);
    }
}

#[derive(Component)]
pub struct Grid2D {
    width: f32,
    height: f32,
    cell_size: f32,
}

fn initialize(mut commands: Commands) {
    commands.spawn(Grid2D::new(15., 15., 50.));
}

impl Grid2D {
    pub fn new(width: f32, height: f32, cell_size: f32) -> Self {
        Self {
            width,
            height,
            cell_size,
        }
    }
}
