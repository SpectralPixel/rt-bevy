pub struct Grid2D {
    width: f32,
    height: f32,
    cell_size: f32,
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
