use bevy::math::Vec2;

#[derive(Debug)]
pub struct Coord(Vec2);

impl Coord {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vec2::new(x, y))
    }

    pub fn floor(&self) -> Self {
        self.0.floor().into()
    }

    pub fn x(&self) -> f32 {
        self.0.x
    }

    pub fn y(&self) -> f32 {
        self.0.y
    }
}

impl From<Vec2> for Coord {
    fn from(v: Vec2) -> Self {
        Self(v)
    }
}

impl Into<Vec2> for Coord {
    fn into(self) -> Vec2 {
        self.0
    }
}
