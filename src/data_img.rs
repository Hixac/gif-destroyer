
macro_rules! vector2d {
	($arr1:expr) => {
		Vector2d::new($arr1[0] as i32, $arr1[1] as i32)
	}
}

pub struct Vector2d {
	pub x: i32,
	pub y: i32,
}

impl Vector2d {
	pub fn new(u: i32, v: i32) -> Self {
		Self {x: u, y: v}
	}
}

pub struct Image {
	pub position: Vector2d,
	pub resolution: Vector2d
}

impl Image {
	pub fn new(pos: Vector2d, res: Vector2d) -> Self {
		Self {position: pos, resolution: res}
	}
}
