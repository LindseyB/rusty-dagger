/* World Module */

extern crate std;
extern crate ncurses;
extern crate rand;

use rand::{task_rng, Rng};
use ncurses::*;
use world::creature::*;

mod creature;

pub struct World {
  pub max_x       : i32,
  pub max_y       : i32,
  pub map_start_x : i32,
  pub map_start_y : i32,
  pub map         : Vec<StrBuf>,
  pub player      : Creature,
  pub enemies     : Vec<Creature>,
  pub msg         : ~str
}

impl World {
  pub fn new(max_x: i32, max_y: i32) -> World {
    let mut map = vec![];
    let mut enemies = vec![];
    /* TODO: generate and fill map based on window size */
    /* fill up our map with dots and walls */
    let width: i32 = 20;
    let height: i32 = 10;
    let map_start_x: i32 = 10;
    let map_start_y: i32 = 10;

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
    let mut player = Creature::new(0, 0, map_start_x, map_start_y, "@", 20, 5, "rusty dagger");

    /* create a single enemy and add it to the list */
    enemies.push(Creature::new(5, 5, map_start_x+5, map_start_y+5, "$", 10, 1, ""));

    World { max_x: max_x, max_y: max_y, map_start_x: map_start_x, map_start_y: map_start_y, map: map, player: player, enemies: enemies, msg: "".to_owned() }
  }

  pub fn draw(&self) {
    clear();
    /* draw the map */
    for i in range(0, self.map.len()) {
      move(self.map_start_y + i as i32, self.map_start_x);
      printw(self.map.get(i as uint).to_str());
    }

    /* draw the hud */
    move(0,0);
    printw(format!("hp    : {:15d} |", self.player.hp));
    move(1,0);
    printw(format!("weapon: {:15s} |", self.player.weapon));
    move(2,0);
    printw(format!("damage: {:15d} |", self.player.damage));
    move(3,0);
    printw("-------------------------");

    /* draw the messages */
    move(self.map_start_y + (self.map.len() as i32) + 3, self.map_start_x);
    printw(self.msg);


    /* move and show the player */
    move(self.player.abs_y, self.player.abs_x);
    attron(COLOR_PAIR(2));
    printw(self.player.pic);

    /* move and show the enemies */
    attron(COLOR_PAIR(3));
    for enemy in self.enemies.iter() {
      if enemy.hp <= 0 {
        // don't draw
      } else {
        move(enemy.abs_y, enemy.abs_x);
        printw(enemy.pic);
      }
    }

    /* reset the colors */
    attron(COLOR_PAIR(1));
  }

  pub fn move_player(&mut self, move_x: i32, move_y: i32) {
    let new_x = self.player.x + move_x;
    let new_y = self.player.y + move_y;

    if new_x >= 0 && new_x < (self.map.get(0).len() as i32) && new_y >= 0 && new_y < (self.map.len() as i32) {
      if self.map.get(new_y as uint).to_str().char_at(new_x as uint) != '.' {
        return;
      }

      /* attacking? */
      for enemy in self.enemies.mut_iter() {
        if new_x == enemy.x && new_y == enemy.y {
          /* attack with 100% success-rate */
          enemy.hp -= self.player.damage;
          if enemy.hp <= 0 {
            self.msg = "You murderer!".to_owned();
          } else {
            self.msg = format!("You attack with {:s} for {:d} damage.", self.player.weapon, self.player.damage);
          }
          return;
        }
      }

      self.player.move(move_x,move_y);
    }
  }

  pub fn update(&mut self) {
    for i in range(0, self.enemies.len()) {
      if self.enemies.get(i).hp <= 0 {
        self.enemies.remove(i);
      } else if (self.enemies.get(i).x == self.player.x && (self.enemies.get(i).y == self.player.y - 1 || self.enemies.get(i).y == self.player.y + 1) ) ||
      (self.enemies.get(i).y == self.player.y && (self.enemies.get(i).x == self.player.x - 1 || self.enemies.get(i).x == self.player.x + 1)) {
        /* is player is adjacent attack */
        self.player.hp -= self.enemies.get(i).damage;
      }

      /* if player is nearby move */

      /* otherwise move randomly */
    }
  }
}
