/* Creature Module */

pub struct Creature {
  pub      x: i32,
  pub      y: i32,
  pub  abs_x: i32,
  pub  abs_y: i32,
  pub    pic: &'static str,
  pub     hp: int,
  pub damage: int,
  pub weapon: &'static str
}

impl Creature {
  pub fn new(x: i32, y: i32, abs_x: i32, abs_y: i32, pic: &'static str, hp: int, damage: int, weapon: &'static str) -> Creature {
    Creature { x: x, y: y, abs_x: abs_x, abs_y: abs_y, pic: pic, hp: hp, damage: damage, weapon: weapon }
  }

  pub fn move(&mut self, move_x: i32, move_y: i32) {
    self.x += move_x;
    self.y += move_y;
    self.abs_x += move_x;
    self.abs_y += move_y;
  }
}


