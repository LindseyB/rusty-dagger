/* Creature Module */

pub struct Creature {
  pub x: i32,
  pub y: i32
}

impl Creature {
  pub fn new(x: i32, y: i32) -> Creature {
    Creature { x: x, y: y }
  }

  pub fn move(&mut self, x: i32, y: i32) {
    self.x = x;
    self.y = y;
  }
}


