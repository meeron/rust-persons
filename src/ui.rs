use ncurses::*;

const ESC: i32 = 27;
const LIST_KEY: i32 = 49;

pub fn ui_init() {
  init();
  menu();
}

fn init() {
  initscr();
  raw();
  keypad(stdscr(), true);
  noecho();
  curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
}

fn menu() {
  clear();
  printw("---Persons database---\n\n");
  printw("1.  Add person\n");
  printw("2.  List\n\n");
  printw("ESC Exit\n");
  refresh();

  loop {
    let key = getch();
    match key {
      LIST_KEY => {
        list();
      }
      ESC => {
        break;
      }
      _ => {}
    }
  }
  endwin();
}

fn list() {
}
