#[derive(Copy, Clone)]
pub struct Position {
  pub x: f64,
  pub y: f64
}

impl PartialEq for Position {
	fn eq(&self, other: &Self) -> bool {
		self.x == other.x && self.y == other.y
	}
}