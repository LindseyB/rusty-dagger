#![feature(globs)]

extern crate ncurses;

use ncurses::*;
use world::*;

mod world;

static KEY_ESC : i32 = 27;

fn main()
{
  /* Start ncurses. */
  initscr();
  keypad(stdscr, true);

  /* Update the screen. */
  refresh();

  let mut max_x = 0;
  let mut max_y = 0;
  getmaxyx(stdscr, &mut max_y, &mut max_x);

  /* create the world */
  let mut world = World::new(max_x, max_y);

  /* main game loop */
  loop {
    /* Wait for a key press. */
    let ch = getch();
    world.draw();

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
