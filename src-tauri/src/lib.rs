use crate::objects::{Book, Music, VideoGame};

mod db;
mod objects;

#[tauri::command(rename_all = "snake_case")]
fn get_book(id: u32) -> Book {
    println!("todo: search for id {}", id);
    Book::default()
}

#[tauri::command(rename_all = "snake_case")]
fn get_music(id: u32) -> Music {
    println!("todo: search for id {}", id);
    Music::default()
}

#[tauri::command(rename_all = "snake_case")]
fn get_videogame(id: u32) -> VideoGame {
    println!("todo: search for id {}", id);
    VideoGame::default()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_book, get_music, get_videogame])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
