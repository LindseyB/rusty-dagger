#![feature(globs)]

extern crate ncurses;

use ncurses::*;

static KEY_ESC : i32 = 27;

fn main()
{
  /* Start ncurses. */
  initscr();
  keypad(stdscr, true);

  /* Print to the back buffer. */
  printw("rusty dagger");

  /* Update the screen. */
  refresh();

  let mut max_x = 0;
  let mut max_y = 0;
  getmaxyx(stdscr, &mut max_y, &mut max_x);
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

  move(10,10);
  printw("@");

  loop {
    /* Wait for a key press. */
    let ch = getch();

    /* exit if esc hit */
    if ch == KEY_ESC {
      endwin();
      break;
    }

    printw(ch.to_str());
  }

  endwin();
}
