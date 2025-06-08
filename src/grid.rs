use crate::prelude::*;
use array2d::Array2D;
use bevy::prelude::*;

pub struct GridPlugin;

pub mod coord;

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
    commands.spawn(Grid2D::new(15, 15, 40));
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

    pub fn cell_size(&self) -> u8 {
        self.cell_size
    }

    pub fn get(&self, pos: &Coord) -> Option<bool> {
        let x = pos.x() as usize;
        let y = pos.y() as usize;
        self.data.get(x, y).cloned()
    }

    pub fn to_grid_pos(&self, world_pos: Vec2) -> Option<Coord> {
        let cell_size = self.cell_size as f32;
        let half_width = self.width() as f32 / 2. * cell_size;
        let half_height = self.height() as f32 / 2. * cell_size;

        if world_pos.x < -half_width
            || world_pos.x > half_width
            || world_pos.y < -half_height
            || world_pos.y > half_height
        {
            return None;
        }

        let x = (world_pos.x + half_width) / cell_size;
        let y = (world_pos.y + half_height) / cell_size;
        Some(Coord::new(x, y))
    }

    pub fn draw_gizmo(&self, gizmos: &mut Gizmos) {
        gizmos.grid_2d(
            Isometry2d::IDENTITY,
            UVec2::from((self.width() as u32, self.height() as u32)),
            Vec2::splat(self.cell_size as f32),
            Color::linear_rgb(0., 1., 0.),
        );

        let mut debug_buffer = String::from("WHERE TRUE: ");
        for col in 0..self.height() {
            for row in 0..self.width() {
                let coord = Coord::new(row as f32, col as f32);
                if self.get(&coord).unwrap() {
                    debug_buffer.push_str(&format!("({:?}) ", coord));
                    use std::f32::consts::SQRT_2;
                    let cell_size = self.cell_size as f32;
                    let half_width = self.width() as f32 / 2.;
                    let half_height = self.height() as f32 / 2.;
                    let x = (coord.x() - half_width + 0.5) * cell_size;
                    let y = (coord.y() - half_height + 0.5) * cell_size;
                    gizmos.cross_2d(
                        Isometry2d::new(Vec2::new(x, y), Rot2::degrees(45.)),
                        cell_size / 2. * SQRT_2,
                        Color::linear_rgb(0., 1., 0.),
                    );
                }
            }
        }
        println!("{}", debug_buffer);
    }
}

fn grid_gizmo(mut gizmos: Gizmos, grid_query: Query<&Grid2D>) {
    for grid in grid_query.iter() {
        grid.draw_gizmo(&mut gizmos);
    }
}
