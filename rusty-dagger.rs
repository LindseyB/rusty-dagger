#![feature(globs)]

extern crate ncurses;

use ncurses::*;

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
  let mut map = vec![];

  struct Creature {
    x: i32,
    y: i32
  };

  impl Creature {
    fn new(x: i32, y: i32) -> Creature {
      Creature { x: x, y: y }
    }

    /* TODO: learn how to just do this from the vars */
    fn move(&mut self, x: i32, y: i32, max_x: i32, max_y: i32) {
      if x >= 0 && x < max_x {
        self.x = x;
      }

      if y >= 0 && y < max_y {
        self.y = y;
      }
    }
  }

  /* create the player */
  let mut player = Creature::new(10,10);

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

  move(player.y,player.x);
  printw("@");

  /* main game loop */
  loop {
    /* Wait for a key press. */
    let ch = getch();

    /* draw the map */
    for i in range(0, max_y) {
      move(i,0);
      /* TODO: index to the correct position in the array */
      printw(map.get(0).to_str());
    }

    /* move and show the player */
    move(player.y, player.x);
    printw("@");

    /* exit if esc hit */
    if ch == KEY_ESC {
      endwin();
      break;
    }

    if ch == KEY_RIGHT {
      player.move(player.x+1, player.y, max_x, max_y);
    }

    if ch == KEY_LEFT {
      player.move(player.x-1, player.y, max_x, max_y);
    }

    if ch == KEY_UP {
      player.move(player.x, player.y-1, max_x, max_y);
    }

    if ch == KEY_DOWN {
      player.move(player.x, player.y+1, max_x, max_y);
    }

    refresh();
  }
}
