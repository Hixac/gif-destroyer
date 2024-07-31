
macro_rules! vector2d {
	($arr1:expr) => {
		Vector2d::new($arr1[0] as i32, $arr1[1] as i32)
	}
}

macro_rules! vector3d {
	($arr1:expr) => {
		{
			let arr = $arr1;
			Vector3d::new(arr[0] as i32, arr[1] as i32, arr[2] as i32)
		}
	}
}

#[derive(Copy, Clone)]
pub struct Vector2d {
	pub x: i32,
	pub y: i32,
}

#[derive(Copy, Clone)]
pub struct Vector3d {
	pub x: i32,
	pub y: i32,
	pub z: i32
}

impl Vector2d {
	pub fn new(u: i32, v: i32) -> Self {
		Self {x: u, y: v}
	}
}

impl Vector3d {
	pub fn new(u: i32, v: i32, g: i32) -> Self {
		Self {x: u, y: v, z: g}
	}
}

pub struct Image {
	pub position: Vector2d,
	pub resolution: Vector2d,
	pub colors: Option<Vec<Vector3d>>
}

impl Image {
	pub fn new(pos: Vector2d, res: Vector2d) -> Self {
		Self {position: pos, resolution: res, colors: None }
	}
}

