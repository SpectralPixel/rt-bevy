use bevy::prelude::*;
use array2d::Array2D;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize);
        //app.add_systems(Update, viewport_gizmo);
    }
}

#[derive(Component)]
pub struct Grid2D {
    data: Array2D<bool>,
    cell_size: u8,
}

fn initialize(mut commands: Commands) {
    commands.spawn(Grid2D::new(15, 15, 5));
}

impl Grid2D {
    pub fn new(width: usize, height: usize, cell_size: u8) -> Self {
        let generator = || rand::random_ratio(1, 5);
        let data = Array2D::filled_by_row_major(generator, width, height);
        Self { data, cell_size }
    }

    pub fn width(&self) -> usize {
        self.data.row_len()
    }

    pub fn height(&self) -> usize {
        self.data.column_len()
    }
}
