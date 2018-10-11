use database::Db;
use ui::*;

pub struct App {
  db: Db,
}

impl App {
  pub fn init() -> App {
    App {
      db: Db::load(String::from("persons.db")),
    }
  }

  pub fn run(self) {
    ui_init();
  }
}
