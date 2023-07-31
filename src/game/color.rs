pub struct Color {
	red: u8,
	green: u8,
	blue: u8,
	alpha: f32
}

#[allow(dead_code)]
impl Color {
	pub const RED: Self = Color {
		red: 255,
		green: 0,
		blue: 0,
		alpha: 1.0
	};

	pub const GREEN: Self = Color {
		red: 0,
		green: 255,
		blue: 0,
		alpha: 1.0
	};

	pub const BLUE: Self = Color {
		red: 0,
		green: 0,
		blue: 255,
		alpha: 1.0
	};

	pub const YELLOW: Self = Color {
		red: 255,
		green: 255,
		blue: 0,
		alpha: 1.0
	};

	pub fn from_rbg(red: u8, green: u8, blue: u8) -> Self {
		Color {
			red,
			green,
			blue,
			alpha: 1.0
		}
	}

	pub fn from_rgba(red: u8, green: u8, blue: u8, alpha: f32) -> Self {
		Color {
			red,
			green,
			blue,
			alpha
		}
	}

	pub fn as_array(&self) -> [f32; 4] {
		let max_u8 = std::u8::MAX;
		[
			self.red as f32 / max_u8 as f32,
			self.green as f32 / max_u8 as f32,
			self.blue as f32 / max_u8 as f32,
			self.alpha
		]
	}
}