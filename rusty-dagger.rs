#![feature(globs)]

extern crate ncurses;

use ncurses::*;

fn main()
{
  /* Start ncurses. */
  initscr();

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
    let mut map_str : ~str= "".to_owned();
    for j in range(0, max_x) {
      map_str = map_str + ".";
      move(i,j);
      printw(".");
    }
    map.push(map_str);
  }

  move(10,10);
  printw("@");

  /* Wait for a key press. */
  getch();

  /* Terminate ncurses. */
  endwin();
}
