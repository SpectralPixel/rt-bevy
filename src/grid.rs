use bevy::prelude::*;
use array2d::Array2D;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize);
        app.add_systems(Update, grid_gizmo);
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

    pub fn draw_gizmo(&self, gizmos: &mut Gizmos) {
        gizmos.grid_2d(Isometry2d::IDENTITY, UVec2::from((self.width() as u32, self.height() as u32)), Vec2::splat(self.cell_size as f32), Color::linear_rgb(0., 1., 0.));
    }
}

fn grid_gizmo(
    mut gizmos: Gizmos,
    grid_query: Query<&Grid2D>,
) {
    for grid in grid_query.iter() {
        grid.draw_gizmo(&mut gizmos);
    }
}
