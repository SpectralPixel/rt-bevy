use crate::prelude::*;
use bevy::prelude::*;

pub struct Ray {
    origin: Vec2,
    direction: Vec2, // is to be normalized
    length: f32,
}

impl Ray {
    pub fn new(origin: Vec2, direction: Vec2) -> Self {
        //assert_eq!(direction.length(), 1.);
        Self {
            origin,
            direction,
            length: 0.,
        }
    }

    pub fn position(&self) -> Vec2 {
        self.origin + self.direction * self.length
    }

    pub fn step(&mut self, grid: &Grid2D) -> Option<bool> {
        let grid_position: Vec2 = grid.to_grid_pos(self.position())?.into();
        let current_cell = grid_position.floor();
        let closest_borders = current_cell + (self.direction / 2.).ceil(); // Division by 2 to ensure that Vec2(-1, 0) is rounded properly
        let delta = closest_borders - grid_position;
        let distance_a = (delta.x / self.direction.x).abs() * grid.cell_size() as f32;
        let distance_b = (delta.y / self.direction.y).abs() * grid.cell_size() as f32;
        self.length += distance_a.min(distance_b) + 0.001;

        let hit_cell = grid.to_grid_pos(self.position())?.floor();
        Some(grid.get(&hit_cell)?)
    }

    pub fn draw_gizmo(&self, gizmos: &mut Gizmos) {
        gizmos.arrow_2d(
            self.origin,
            self.origin + self.direction * self.length,
            Color::linear_rgb(1., 0., 0.),
        );
    }
}
