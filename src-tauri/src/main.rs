use serde::{Deserialize, Serialize};
use tauri;

use std::sync::{Arc, Mutex};

#[tauri::command]
async fn my_command(state: tauri::State<'_, Database>, message: String) -> Result<String, String> {
  let counter = state.arcmut.lock().unwrap().count;
  println!("got message {} (counter: {})", message, counter);

  Ok(format!("star says (counter: {}): {}.", counter, message).into())
}

#[derive(Default)]
struct MyState {
  count: i32,
}

impl MyState {
  fn new(count: i32) -> MyState {
    MyState { count: count }
  }
}

struct Database {
  arcmut: Arc<Mutex<MyState>>,
}

impl Database {
  fn new(count: i32) -> Database {
    Database {
      arcmut: Arc::new(Mutex::new(MyState::new(3))),
    }
  }
}
use std::fs;

#[tauri::command]
fn check_my_state(state: tauri::State<'_, Database>) {
  //println!("state: {}", state.count);
  let mut my_state = state.arcmut.lock().unwrap();
  my_state.count += 1;
  println!("state: {}", my_state.count);

  let paths = fs::read_dir("./").unwrap();

  for path in paths {
    println!("Name: {}", path.unwrap().path().display())
  }
}

#[derive(Serialize, Deserialize)]
struct Dir {
  files: std::vec::Vec<String>,
  path: String,
}

impl Dir {
  fn new() -> Self {
    Self {
      files: vec!["file1".into(), "file2".into(), "file3".into()],
      path: ".".into(),
    }
  }
}

#[tauri::command]
fn get_files() -> Result<Dir, String> {
  // let paths = fs::read_dir("./").unwrap();

  // for path in paths {
  //   println!("Name: {}", path.unwrap().path().display())
  // }
  let mut dir = Dir::new();
  dir.files = fs::read_dir("./")
    .unwrap()
    .map(|r| r.unwrap().path().into_os_string().into_string().unwrap())
    .collect::<Vec<String>>();
  Ok(dir)
}

fn main() {
  tauri::Builder::default()
    .manage(Database::new(5))
    .invoke_handler(tauri::generate_handler![
      my_command,
      check_my_state,
      get_files
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
