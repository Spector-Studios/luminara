use macroquad::math::Vec2;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TileRect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Point {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub const fn zero() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl TileRect {
    pub fn with_size(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self { x, y, w, h }
    }

    pub fn point_in_rect(&self, pt: Point) -> bool {
        self.x <= pt.x && self.y <= pt.y && (self.x + self.w) > pt.x && (self.y + self.h) > pt.y
    }

    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(Point),
    {
        for y in self.y..self.y + self.h {
            for x in self.x..self.x + self.w {
                f(Point { x, y });
            }
        }
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl From<(i32, i32)> for Point {
    fn from(value: (i32, i32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

#[allow(clippy::cast_precision_loss)]
impl From<Point> for Vec2 {
    fn from(val: Point) -> Self {
        Vec2::new(val.x as f32, val.y as f32)
    }
}
