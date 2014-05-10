#![feature(globs)]

extern crate ncurses;
extern crate rand;

use ncurses::*;
use world::*;
use rand::{task_rng, Rng};

mod world;

static KEY_ESC : i32 = 27;

fn main()
{
  /* Start ncurses. */
  initscr();
  start_color();
  keypad(stdscr, true);

  /* setup some colors */
  init_pair(1, COLOR_WHITE, COLOR_BLACK);
  init_pair(2, COLOR_GREEN, COLOR_BLACK);
  init_pair(3, COLOR_RED, COLOR_BLACK);

  /* Update the screen. */
  refresh();

  let mut max_x = 0;
  let mut max_y = 0;
  getmaxyx(stdscr, &mut max_y, &mut max_x);

  /* create the world */
  let mut world = World::new(max_x, max_y);
  /* main game loop */
  loop {
    world.draw();
    let ch = getch(); /* get keypress */

    /* exit if esc hit */
    if ch == KEY_ESC {
      endwin();
      break;
    }

    if ch == KEY_RIGHT {
      world.move_player(world.player.x+1, world.player.y);
    }

    if ch == KEY_LEFT {
      world.move_player(world.player.x-1, world.player.y);
    }

    if ch == KEY_UP {
      world.move_player(world.player.x, world.player.y-1);
    }

    if ch == KEY_DOWN {
      world.move_player(world.player.x, world.player.y+1);
    }

    refresh();
  }
}
