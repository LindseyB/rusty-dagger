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

  /* Wait for a key press. */
  getch();

  /* Terminate ncurses. */
  endwin();
}
