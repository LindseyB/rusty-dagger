/* World Module */

extern crate std;
extern crate ncurses;
extern crate rand;

use rand::{task_rng, Rng};
use ncurses::*;
use world::creature::*;

mod creature;

pub struct World {
  pub max_x: i32,
  pub max_y: i32,
  pub map_start_x: i32,
  pub map_start_y: i32,
  pub map: std::vec::Vec<std::strbuf::StrBuf>,
  pub player: Creature
}

impl World {
  pub fn new(max_x: i32, max_y: i32) -> World {
    let mut map = vec![];
    /* TODO: generate and fill map based on window size */
    /* fill up our map with dots */
    let width: i32 = 20;
    let height: i32 = 10;

    for i in range(0, height) {
      let mut map_str = StrBuf::from_str("");
      for j in range(0, width) {
        let num = task_rng().gen_range(0,10);
        if num == 0 {
          map_str.push_char('#');
        } else {
          map_str.push_char('.');
        }
      }
      move(i,0);
      printw(map_str.to_str());
      map.push(map_str);
    }

    /* create the player */
    let mut player = Creature::new(10, 10, "@", 20, 5, "rusty dagger");

    World { max_x: max_x, max_y: max_y, map_start_x: 10, map_start_y: 10, map: map, player: player }
  }

  pub fn draw(&self) {
    clear();
    /* draw the map */
    for i in range(0, self.map.len()) {
      move(self.map_start_y + i as i32, self.map_start_x);
      printw(self.map.get(i as uint).to_str());
    }

    /* move and show the player */
    move(self.player.y, self.player.x);
    printw(self.player.pic);

    /* draw the hud */
    move(0,0);
    printw(format!("hp    : {:15d} |", self.player.hp));
    move(1,0);
    printw(format!("weapon: {:15s} |", self.player.weapon));
    move(2,0);
    printw(format!("damage: {:15d} |", self.player.damage));
    move(3,0);
    printw("-------------------------");
  }

  pub fn move_player(&mut self, x: i32, y: i32) {
    if x >= self.map_start_x && x < (self.map_start_x + self.map.get(0).len() as i32) && y >= self.map_start_y && y < (self.map_start_y + self.map.len() as i32) {
      self.player.move(x,y);
    }
  }
}
