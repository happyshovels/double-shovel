use serde::{Deserialize, Serialize};
use tauri::{self, Manager, State, Window};

use std::sync::{Arc, Mutex};

extern crate open;

struct SplashscreenWindow(Arc<Mutex<Window>>);
struct MainWindow(Arc<Mutex<Window>>);

#[tauri::command]
fn close_splashscreen(
  splashscreen: State<SplashscreenWindow>,
  main: State<MainWindow>,
) {
  // Close splashscreen
  splashscreen.0.lock().unwrap().close().unwrap();
  // Show main window
  main.0.lock().unwrap().show().unwrap();
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

#[derive(Serialize, Deserialize)]
struct DirEntry {
  name: String,
  entry_type: String,
}



#[derive(Serialize, Deserialize)]
struct Dir {
  files: std::vec::Vec<DirEntry>,
  path: String,
}



impl Dir {
  fn new() -> Self {
    Self {
      files: vec![],
      path: ".".into(),
    }
  }
}

#[tauri::command]
fn open_file(file_path: String) -> Result<String, String> {

  println!("open: {}", file_path);
  open::that(file_path);
  Ok("ok".into())
}

#[tauri::command]
fn get_folder_content(query_path: String) -> Result<Dir, String> {
  let mut dir = Dir::new();
  // todo include some error handling

  let files = fs::read_dir(query_path).unwrap();
  dir.files = files
      .map(|r| {
          let pathbuf = r.unwrap().path();

          DirEntry {
              name: pathbuf.file_name().unwrap().to_str().unwrap().into(),
              entry_type: if pathbuf.is_dir() {
                  "dir".into()
              } else {
                  "file".into()
              },
          }
      })
      .collect::<Vec<DirEntry>>();


  Ok(dir)
}

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      // set the splashscreen and main windows to be globally available with the tauri state API
      app.manage(SplashscreenWindow(Arc::new(Mutex::new(
        app.get_window("splashscreen").unwrap(),
      ))));
      app.manage(MainWindow(Arc::new(Mutex::new(
        app.get_window("main").unwrap(),
      ))));
      Ok(())
    })
    .manage(Database::new(5))
    .invoke_handler(tauri::generate_handler![
      get_folder_content,
      open_file,
      close_splashscreen
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
