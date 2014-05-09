/* Creature Module */

pub struct Creature {
  pub      x: i32,
  pub      y: i32,
  pub    pic: &'static str,
  pub     hp: int,
  pub damage: int,
  pub weapon: &'static str
}

impl Creature {
  pub fn new(x: i32, y: i32, pic: &'static str, hp: int, damage: int, weapon: &'static str) -> Creature {
    Creature { x: x, y: y, pic: pic, hp: hp, damage: damage, weapon: weapon }
  }

  pub fn move(&mut self, x: i32, y: i32) {
    self.x = x;
    self.y = y;
  }
}


