use std::ops::DerefMut;

use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use tauri::{Manager, State};

use crate::{
    db::models::{Book, Music, Videogame},
    state::AppState,
};
use diesel::delete;
use diesel::insert_into;
use diesel::update;

use diesel::prelude::*;

mod db;
mod prelude;
mod state;
use prelude::*;

const PAGE_SIZE: i64 = 10;

#[tauri::command(rename_all = "snake_case")]
fn get_musics(page: i64, state: State<'_, AppState>) -> Vec<Music> {
    use db::schema::musica::dsl::*;

    let offset = page * PAGE_SIZE;
    let result: Vec<Music> = {
        let mut binding = state.posgresql.lock().unwrap();
        info!("started");
        let connection = binding.deref_mut();
        musica
            .offset(offset)
            .limit(PAGE_SIZE)
            .select(Music::as_select())
            .load(connection)
            .expect("Error loading musics")
    };

    result
}

#[tauri::command(rename_all = "snake_case")]
fn get_videogames(page: i64, state: State<'_, AppState>) -> Vec<Videogame> {
    use db::schema::videojuego::dsl::*;

    let offset = page * PAGE_SIZE;
    let result: Vec<Videogame> = {
        let mut binding = state.posgresql.lock().unwrap();
        info!("started");
        let connection = binding.deref_mut();
        videojuego
            .offset(offset)
            .limit(PAGE_SIZE)
            .select(Videogame::as_select())
            .load(connection)
            .expect("Error loading videogames")
    };

    result
}

#[tauri::command(rename_all = "snake_case")]
fn get_books(page: i64, state: State<'_, AppState>) -> Vec<Book> {
    use db::schema::libro::dsl::*;

    let offset = page * PAGE_SIZE;
    let result: Vec<Book> = {
        let mut binding = state.posgresql.lock().unwrap();
        info!("started");
        let connection = binding.deref_mut();
        libro
            .offset(offset)
            .limit(PAGE_SIZE)
            .select(Book::as_select())
            .load(connection)
            .expect("Error loading books")
    };

    result
}

#[tauri::command(rename_all = "snake_case")]
fn update_book(new_book: Book, state: State<'_, AppState>) -> bool {
    use db::schema::libro::dsl::*;
    let mut binding = state.posgresql.lock().unwrap();
    let connection = binding.deref_mut();
    let result = update(libro.filter(id_objeto.eq(&new_book.object_id.trim())))
        .set((
            nombre_objeto.eq(&new_book.object_name.trim()),
            ano_publicacion.eq(&new_book.publication_year),
            id_identificador.eq(&new_book.identifier_id.trim()),
            id_idioma.eq(&new_book.language_id.trim()),
            cantidad_de_paginas.eq(new_book.page_count),
        ))
        .execute(connection);
    match result {
        Ok(rows) => rows > 0,
        Err(_) => false,
    }
}

#[tauri::command(rename_all = "snake_case")]
fn update_music(new_music: Music, state: State<'_, AppState>) -> bool {
    use db::schema::musica::dsl::*;
    let mut binding = state.posgresql.lock().unwrap();
    let connection = binding.deref_mut();
    let result = update(musica.filter(id_objeto.eq(&new_music.object_id.trim())))
        .set((
            nombre_objeto.eq(&new_music.object_name.trim()),
            ano_publicacion.eq(&new_music.publication_year),
            id_identificador.eq(&new_music.identifier_id.trim()),
            id_idioma.eq(&new_music.language_id.trim()),
            duracion.eq(new_music.duration),
        ))
        .execute(connection);
    match result {
        Ok(rows) => rows > 0,
        Err(e) => {
            error!("UpdateError: {}", e);
            false
        }
    }
}

#[tauri::command(rename_all = "snake_case")]
fn update_videogame(new_videogame: Videogame, state: State<'_, AppState>) -> bool {
    use db::schema::videojuego::dsl::*;
    let mut binding = state.posgresql.lock().unwrap();
    let connection = binding.deref_mut();
    let result = update(videojuego.filter(id_objeto.eq(&new_videogame.object_id.trim())))
        .set((
            nombre_objeto.eq(&new_videogame.object_name.trim()),
            ano_publicacion.eq(&new_videogame.publication_year),
            id_identificador.eq(&new_videogame.identifier_id.trim()),
            id_idioma.eq(&new_videogame.language_id.trim()),
            soporte_de_mando.eq(new_videogame.controller_support),
            id_pegi.eq(&new_videogame.pegi_id.trim()),
        ))
        .execute(connection);
    match result {
        Ok(rows) => rows > 0,
        Err(e) => {
            error!("UpdateError: {}", e);
            false
        }
    }
}

#[tauri::command(rename_all = "snake_case")]
fn delete_book(id: String, state: State<'_, AppState>) -> bool {
    use db::schema::libro::dsl::*;
    let mut binding = state.posgresql.lock().unwrap();
    let connection = binding.deref_mut();
    let result = delete(libro.filter(id_objeto.eq(id.trim()))).execute(connection);
    match result {
        Ok(rows) => rows > 0,
        Err(_) => false,
    }
}

#[tauri::command(rename_all = "snake_case")]
fn delete_music(id: String, state: State<'_, AppState>) -> bool {
    use db::schema::musica::dsl::*;
    let mut binding = state.posgresql.lock().unwrap();
    let connection = binding.deref_mut();
    let result = delete(musica.filter(id_objeto.eq(id.trim()))).execute(connection);
    match result {
        Ok(rows) => rows > 0,
        Err(_) => false,
    }
}

#[tauri::command(rename_all = "snake_case")]
fn delete_videogame(id: String, state: State<'_, AppState>) -> bool {
    use db::schema::videojuego::dsl::*;
    let mut binding = state.posgresql.lock().unwrap();
    let connection = binding.deref_mut();
    let result = delete(videojuego.filter(id_objeto.eq(id.trim()))).execute(connection);
    match result {
        Ok(rows) => rows > 0,
        Err(_) => false,
    }
}

#[tauri::command(rename_all = "snake_case")]
fn create_book(new_book: Book, state: State<'_, AppState>) -> bool {
    use db::schema::libro::dsl::*;
    let mut binding = state.posgresql.lock().unwrap();
    let connection = binding.deref_mut();
    let result = insert_into(libro)
        .values((
            id_objeto.eq(&new_book.object_id),
            nombre_objeto.eq(&new_book.object_name),
            ano_publicacion.eq(&new_book.publication_year),
            id_identificador.eq(&new_book.identifier_id),
            id_idioma.eq(&new_book.language_id),
            cantidad_de_paginas.eq(new_book.page_count),
        ))
        .execute(connection);
    match result {
        Ok(rows) => rows > 0,
        Err(_) => false,
    }
}

#[tauri::command(rename_all = "snake_case")]
fn create_music(new_music: Music, state: State<'_, AppState>) -> bool {
    use db::schema::musica::dsl::*;
    let mut binding = state.posgresql.lock().unwrap();
    let connection = binding.deref_mut();
    let result = insert_into(musica)
        .values((
            id_objeto.eq(&new_music.object_id),
            nombre_objeto.eq(&new_music.object_name),
            ano_publicacion.eq(&new_music.publication_year),
            id_identificador.eq(&new_music.identifier_id),
            id_idioma.eq(&new_music.language_id),
            duracion.eq(new_music.duration),
        ))
        .execute(connection);
    match result {
        Ok(rows) => rows > 0,
        Err(_) => false,
    }
}

#[tauri::command(rename_all = "snake_case")]
fn create_videogame(new_videogame: Videogame, state: State<'_, AppState>) -> bool {
    use db::schema::videojuego::dsl::*;
    let mut binding = state.posgresql.lock().unwrap();
    let connection = binding.deref_mut();
    let result = insert_into(videojuego)
        .values((
            id_objeto.eq(&new_videogame.object_id),
            nombre_objeto.eq(&new_videogame.object_name),
            ano_publicacion.eq(&new_videogame.publication_year),
            id_identificador.eq(&new_videogame.identifier_id),
            id_idioma.eq(&new_videogame.language_id),
            soporte_de_mando.eq(new_videogame.controller_support),
            id_pegi.eq(&new_videogame.pegi_id),
        ))
        .execute(connection);
    match result {
        Ok(rows) => rows > 0,
        Err(_) => false,
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    TermLogger::init(
        log::LevelFilter::Debug,
        Config::default(),
        TerminalMode::Stderr,
        ColorChoice::Auto,
    )
    .expect("This should be unreachable");
    tauri::Builder::default()
        .setup(|app| {
            app.manage(AppState::new()?);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_books,
            get_musics,
            get_videogames,
            update_book,
            update_music,
            update_videogame,
            delete_book,
            delete_music,
            delete_videogame,
            create_book,
            create_music,
            create_videogame
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
