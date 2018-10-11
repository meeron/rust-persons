extern crate ncurses;

mod app;
mod database;
mod ui;

use app::App;

fn main() {
  let app = App::init();
  app.run();
}
