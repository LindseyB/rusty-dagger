/* Goal Module */

pub struct Goal {
  pub      x: i32,
  pub      y: i32,
  pub  abs_x: i32,
  pub  abs_y: i32,
  pub  got: bool
}

impl Goal {
	pub fn new(x: i32, y: i32, abs_x: i32, abs_y: i32) -> Goal {
		Goal { x: x, y: y, abs_x: abs_x, abs_y: abs_y, got: false }
	}
}