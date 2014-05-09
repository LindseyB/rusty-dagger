
extern crate std;
extern crate ncurses;

use ncurses::*;
use world::creature::*;

mod creature;

pub struct World {
  pub max_x: i32,
  pub max_y: i32,
  pub map: std::vec::Vec<std::strbuf::StrBuf>,
  pub player: Creature
}

impl World {
  pub fn new(max_x: i32, max_y: i32) -> World {
    let mut map = vec![];
    /* fill up our map and screen with dots */
    for i in range(0, max_y) {
      let mut map_str = StrBuf::from_str("");
      for j in range(0, max_x) {
        map_str.push_char('.');
      }
      move(i,0);
      printw(map_str.to_str());
      map.push(map_str);
    }

    /* create the player */
    let mut player = Creature::new(10, 10);

    World { max_x: max_x, max_y: max_y, map: map, player: player }
  }

  pub fn draw(&self) {
    /* draw the map */
    for i in range(0, self.max_y) {
      move(i,0);
      /* TODO: index to the correct position in the array */
      printw(self.map.get(0).to_str());
    }

    /* move and show the player */
    move(self.player.y, self.player.x);
    printw("@");
  }

  pub fn move_player(&mut self, x: i32, y: i32) {
    if x >= 0 && x < self.max_x && y >= 0 && y < self.max_y {
      self.player.move(x,y);
    } 
  }
}
