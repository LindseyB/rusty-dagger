/* World Module */

extern crate std;
extern crate ncurses;
extern crate rand;

use rand::{task_rng, Rng};
use ncurses::*;
use world::creature::*;
use world::goal::*;

mod creature;
mod goal;

pub struct World {
  pub max_x       : i32,
  pub max_y       : i32,
  pub map_start_x : i32,
  pub map_start_y : i32,
  pub map         : Vec<StrBuf>,
  pub player      : Creature,
  pub enemies     : Vec<Creature>,
  pub goal        : Goal,
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

    /* TODO: add more enemies */
    /* create a single enemy and add it to the list */
    enemies.push(Creature::new(5, 5, map_start_x+5, map_start_y+5, "$", 10, 2, ""));

    /* create the goal */
    let mut goal = Goal::new(width-1, height-1, map_start_x+width-1, map_start_y+height-1);

    World { max_x: max_x, max_y: max_y, map_start_x: map_start_x, map_start_y: map_start_y, map: map, player: player, enemies: enemies, msg: "".to_owned(), goal: goal }
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

    /* TODO: only show enemies if in player draw distance */
    /* show the enemies */
    attron(COLOR_PAIR(3));
    for enemy in self.enemies.iter() {
      move(enemy.abs_y, enemy.abs_x);
      printw(enemy.pic);
    }

    if !self.goal.got {
      /* show the goal */
      attron(COLOR_PAIR(4));
      move(self.goal.abs_y, self.goal.abs_x);
      printw("*");
    }

    /* reset the colors */
    attron(COLOR_PAIR(1));
  }

  pub fn move_player(&mut self, move_x: i32, move_y: i32) {
    let new_x = self.player.x + move_x;
    let new_y = self.player.y + move_y;

    /* if player dead or won do nothing */
    if self.player.hp <= 0 || self.goal.got {
      return;
    }

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

      /* goal get? */
      if new_x == self.goal.x && new_y == self.goal.y {
        self.msg = "Goal get! You win!".to_owned();
        self.goal.got = true;
      }

      self.player.move(move_x,move_y);
    }
  }

  pub fn update(&mut self) {
    /* if player dead or won do nothing */
    if self.player.hp <= 0 || self.goal.got {
      return;
    }

    /* remove dead enemies */
    for i in range(0, self.enemies.len()) {
      if self.enemies.get(i).hp <= 0 {
        self.enemies.remove(i);
      }
    }

    for enemy in self.enemies.mut_iter() {
      if (enemy.x == self.player.x && (enemy.y == self.player.y - 1 || enemy.y == self.player.y + 1) ) ||
      (enemy.y == self.player.y && (enemy.x == self.player.x - 1 || enemy.x == self.player.x + 1)) {
        /* is player is adjacent attack */
        self.player.hp -= enemy.damage;

        /* check if played dead */
        if self.player.hp <= 0 {
          self.msg = "You die".to_owned();
          self.player.pic = "^";
        }
      } else {
        /* make a move */
        let mut new_x = enemy.x;
        let mut new_y = enemy.y;
        let mut move_x = 0;
        let mut move_y = 0;

        if enemy.x >= self.player.x - 3 &&
                  enemy.x <= self.player.x + 3 &&
                  enemy.y >= self.player.y - 3 &&
                  enemy.y <= self.player.y + 3 {
          /* if player is nearby move towards player */
          if enemy.x >= self.player.x {
            /* move left */
            move_x = -1;
          } else if enemy.x <= self.player.x {
            /* move right */
            move_x = 1;
          } else if enemy.y <= self.player.y {
            /* move up */
            move_y = 1;
          } else {
            /* move up */
            move_y = -1;
          }
        } else {
          /* otherwise move randomly */
          let num = task_rng().gen_range(0,4);

          if num == 0 {
            /* move up */
            move_y = -1;
          } else if num == 1 {
            /* move down */
            move_y = 1;
          } else if num == 2 {
            /* move left */
            move_x = -1;
          } else {
            /* move right */
            move_x = 1;
          }

          new_x += move_x;
          new_y += move_y;
        }

        /* check if valid move and make it */
        if new_x >= 0 && new_x < (self.map.get(0).len() as i32) && new_y >= 0 && new_y < (self.map.len() as i32) {
          if self.map.get(new_y as uint).to_str().char_at(new_x as uint) == '.' {
            enemy.move(move_x,move_y);
          }
        }
      }
    }
  }
}
