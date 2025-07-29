use std::ops::DerefMut;

use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use tauri::{Manager, State};

use crate::{
    db::models::{
        Author, Book, Collection, Country, Distributor, Genre, Identifier, Language, Music, Pegi,
        Studio, Videogame,
    },
    state::AppState,
};
use diesel::insert_into;
use diesel::update;
use diesel::{delete, dsl::count};

use diesel::prelude::*;

mod db;
mod prelude;
mod state;
use prelude::*;

const PAGE_SIZE: i64 = 10;

#[tauri::command(rename_all = "snake_case")]
fn get_distributors(page: i64, state: State<'_, AppState>) -> Vec<Distributor> {
    use db::schema::distribuidor::dsl::*;
    let offset = page * PAGE_SIZE;
    let result: Vec<Distributor> = {
        let mut binding = state.posgresql.lock().unwrap();
        info!("get_distributors: page={}", page);
        let connection = binding.deref_mut();
        distribuidor
            .offset(offset)
            .limit(PAGE_SIZE)
            .select(Distributor::as_select())
            .load(connection)
            .expect("Error loading distributors")
    };
    result
}

#[tauri::command(rename_all = "snake_case")]
fn update_distributor(new_distributor: Distributor, state: State<'_, AppState>) -> bool {
    use db::schema::distribuidor::dsl::*;
    let mut binding = state.posgresql.lock().unwrap();
    let connection = binding.deref_mut();
    let result =
        update(distribuidor.filter(id_distribuidor.eq(&new_distributor.distributor_id.trim())))
            .set((
                id_pais.eq(&new_distributor.country_id.trim()),
                nombre_distribuidor.eq(&new_distributor.distributor_name.trim()),
                ano_fundacion.eq(&new_distributor.foundation_year),
            ))
            .execute(connection);
    match result {
        Ok(rows) => rows > 0,
        Err(_) => false,
    }
}

#[tauri::command(rename_all = "snake_case")]
fn delete_distributor(id: String, state: State<'_, AppState>) -> bool {
    use db::schema::distribuidor::dsl::*;
    let mut binding = state.posgresql.lock().unwrap();
    let connection = binding.deref_mut();
    let result = delete(distribuidor.filter(id_distribuidor.eq(id.trim()))).execute(connection);
    match result {
        Ok(rows) => rows > 0,
        Err(_) => false,
    }
}

#[tauri::command(rename_all = "snake_case")]
fn create_distributor(new_distributor: Distributor, state: State<'_, AppState>) -> bool {
    use db::schema::distribuidor::dsl::*;
    let mut binding = state.posgresql.lock().unwrap();
    let connection = binding.deref_mut();
    let result = insert_into(distribuidor)
        .values((
            id_distribuidor.eq(&new_distributor.distributor_id),
            id_pais.eq(&new_distributor.country_id),
            nombre_distribuidor.eq(&new_distributor.distributor_name),
            ano_fundacion.eq(&new_distributor.foundation_year),
        ))
        .execute(connection);
    match result {
        Ok(rows) => rows > 0,
        Err(_) => false,
    }
}

#[tauri::command(rename_all = "snake_case")]
fn get_genres(page: i64, state: State<'_, AppState>) -> Vec<Genre> {
    use db::schema::genero::dsl::*;
    let offset = page * PAGE_SIZE;
    let result: Vec<Genre> = {
        let mut binding = state.posgresql.lock().unwrap();
        info!("get_genres: page={}", page);
        let connection = binding.deref_mut();
        genero
            .offset(offset)
            .limit(PAGE_SIZE)
            .select(Genre::as_select())
            .load(connection)
            .expect("Error loading genres")
    };
    result
}

#[tauri::command(rename_all = "snake_case")]
fn update_genre(new_genre: Genre, state: State<'_, AppState>) -> bool {
    use db::schema::genero::dsl::*;
    let mut binding = state.posgresql.lock().unwrap();
    let connection = binding.deref_mut();
    let result = update(genero.filter(id_genero.eq(&new_genre.genre_id.trim())))
        .set((nombre_genero.eq(&new_genre.genre_name.trim()),))
        .execute(connection);
    match result {
        Ok(rows) => rows > 0,
        Err(_) => false,
    }
}

#[tauri::command(rename_all = "snake_case")]
fn delete_genre(id: String, state: State<'_, AppState>) -> bool {
    use db::schema::genero::dsl::*;
    let mut binding = state.posgresql.lock().unwrap();
    let connection = binding.deref_mut();
    let result = delete(genero.filter(id_genero.eq(id.trim()))).execute(connection);
    match result {
        Ok(rows) => rows > 0,
        Err(_) => false,
    }
}

#[tauri::command(rename_all = "snake_case")]
fn create_genre(new_genre: Genre, state: State<'_, AppState>) -> bool {
    use db::schema::genero::dsl::*;
    let mut binding = state.posgresql.lock().unwrap();
    let connection = binding.deref_mut();
    let result = insert_into(genero)
        .values((
            id_genero.eq(&new_genre.genre_id),
            nombre_genero.eq(&new_genre.genre_name),
        ))
        .execute(connection);
    match result {
        Ok(rows) => rows > 0,
        Err(_) => false,
    }
}

#[tauri::command(rename_all = "snake_case")]
fn get_musics(page: i64, state: State<'_, AppState>) -> Vec<Music> {
    use db::schema::musica::dsl::*;

    let offset = page * PAGE_SIZE;
    let result: Vec<Music> = {
        let mut binding = state.posgresql.lock().unwrap();
        info!("get_musics: page={}", page);
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
        info!("get_videogames: page={}", page);
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
        info!("get_books: page={}", page);
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
fn get_book_by_name(page: i64, name: String, state: State<'_, AppState>) -> Vec<Book> {
    use db::schema::libro::dsl::*;
    let offset = page * PAGE_SIZE;
    let mut binding = state.posgresql.lock().unwrap();
    let connection = binding.deref_mut();
    libro
        .filter(nombre_objeto.ilike(format!("%{}%", name.trim())))
        .offset(offset)
        .limit(PAGE_SIZE)
        .select(Book::as_select())
        .load(connection)
        .unwrap_or_default()
}

#[tauri::command(rename_all = "snake_case")]
fn get_music_by_name(page: i64, name: String, state: State<'_, AppState>) -> Vec<Music> {
    use db::schema::musica::dsl::*;
    let offset = page * PAGE_SIZE;
    let mut binding = state.posgresql.lock().unwrap();
    let connection = binding.deref_mut();
    musica
        .filter(nombre_objeto.ilike(format!("%{}%", name.trim())))
        .offset(offset)
        .limit(PAGE_SIZE)
        .select(Music::as_select())
        .load(connection)
        .unwrap_or_default()
}

#[tauri::command(rename_all = "snake_case")]
fn get_videogame_by_name(page: i64, name: String, state: State<'_, AppState>) -> Vec<Videogame> {
    use db::schema::videojuego::dsl::*;
    let offset = page * PAGE_SIZE;
    let mut binding = state.posgresql.lock().unwrap();
    let connection = binding.deref_mut();
    videojuego
        .filter(nombre_objeto.ilike(format!("%{}%", name.trim())))
        .offset(offset)
        .limit(PAGE_SIZE)
        .select(Videogame::as_select())
        .load(connection)
        .unwrap_or_default()
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

#[tauri::command(rename_all = "snake_case")]
fn get_identifiers(page: i64, state: State<'_, AppState>) -> Vec<Identifier> {
    use db::schema::identificador::dsl::*;
    let offset = page * PAGE_SIZE;
    let result: Vec<Identifier> = {
        let mut binding = state.posgresql.lock().unwrap();
        info!("get_identifiers: page={}", page);
        let connection = binding.deref_mut();
        identificador
            .offset(offset)
            .limit(PAGE_SIZE)
            .select(Identifier::as_select())
            .load(connection)
            .expect("Error loading identifiers")
    };
    result
}

#[tauri::command(rename_all = "snake_case")]
fn update_identifier(new_identifier: Identifier, state: State<'_, AppState>) -> bool {
    use db::schema::identificador::dsl::*;
    let mut binding = state.posgresql.lock().unwrap();
    let connection = binding.deref_mut();
    let result =
        update(identificador.filter(id_identificador.eq(&new_identifier.identifier_id.trim())))
            .set((identificador_dewey_de_objeto.eq(&new_identifier.dewey_identifier.trim()),))
            .execute(connection);
    match result {
        Ok(rows) => rows > 0,
        Err(_) => false,
    }
}

#[tauri::command(rename_all = "snake_case")]
fn delete_identifier(id: String, state: State<'_, AppState>) -> bool {
    use db::schema::identificador::dsl::*;
    let mut binding = state.posgresql.lock().unwrap();
    let connection = binding.deref_mut();
    let result = delete(identificador.filter(id_identificador.eq(id.trim()))).execute(connection);
    match result {
        Ok(rows) => rows > 0,
        Err(_) => false,
    }
}

#[tauri::command(rename_all = "snake_case")]
fn create_identifier(new_identifier: Identifier, state: State<'_, AppState>) -> bool {
    use db::schema::identificador::dsl::*;
    let mut binding = state.posgresql.lock().unwrap();
    let connection = binding.deref_mut();
    let result = insert_into(identificador)
        .values((
            id_identificador.eq(&new_identifier.identifier_id),
            identificador_dewey_de_objeto.eq(&new_identifier.dewey_identifier),
        ))
        .execute(connection);
    match result {
        Ok(rows) => rows > 0,
        Err(_) => false,
    }
}

#[tauri::command(rename_all = "snake_case")]
fn get_languages(page: i64, state: State<'_, AppState>) -> Vec<Language> {
    use db::schema::idioma::dsl::*;
    let offset = page * PAGE_SIZE;
    let result: Vec<Language> = {
        let mut binding = state.posgresql.lock().unwrap();
        info!("get_languages: page={}", page);
        let connection = binding.deref_mut();
        idioma
            .offset(offset)
            .limit(PAGE_SIZE)
            .select(Language::as_select())
            .load(connection)
            .expect("Error loading languages")
    };
    result
}

#[tauri::command(rename_all = "snake_case")]
fn update_language(new_language: Language, state: State<'_, AppState>) -> bool {
    use db::schema::idioma::dsl::*;
    let mut binding = state.posgresql.lock().unwrap();
    let connection = binding.deref_mut();
    let result = update(idioma.filter(id_idioma.eq(&new_language.language_id.trim())))
        .set((nombre_idioma.eq(&new_language.language_name.trim()),))
        .execute(connection);
    match result {
        Ok(rows) => rows > 0,
        Err(_) => false,
    }
}

#[tauri::command(rename_all = "snake_case")]
fn delete_language(id: String, state: State<'_, AppState>) -> bool {
    use db::schema::idioma::dsl::*;
    let mut binding = state.posgresql.lock().unwrap();
    let connection = binding.deref_mut();
    let result = delete(idioma.filter(id_idioma.eq(id.trim()))).execute(connection);
    match result {
        Ok(rows) => rows > 0,
        Err(_) => false,
    }
}

#[tauri::command(rename_all = "snake_case")]
fn create_language(new_language: Language, state: State<'_, AppState>) -> bool {
    use db::schema::idioma::dsl::*;
    let mut binding = state.posgresql.lock().unwrap();
    let connection = binding.deref_mut();
    let result = insert_into(idioma)
        .values((
            id_idioma.eq(&new_language.language_id),
            nombre_idioma.eq(&new_language.language_name),
        ))
        .execute(connection);
    match result {
        Ok(rows) => rows > 0,
        Err(_) => false,
    }
}
#[tauri::command(rename_all = "snake_case")]
fn get_countries(page: i64, state: State<'_, AppState>) -> Vec<Country> {
    use db::schema::pais::dsl::*;
    let offset = page * PAGE_SIZE;
    let result: Vec<Country> = {
        let mut binding = state.posgresql.lock().unwrap();
        info!("get_countries: page={}", page);
        let connection = binding.deref_mut();
        pais.offset(offset)
            .limit(PAGE_SIZE)
            .select(Country::as_select())
            .load(connection)
            .expect("Error loading countries")
    };
    result
}

#[tauri::command(rename_all = "snake_case")]
fn update_country(new_country: Country, state: State<'_, AppState>) -> bool {
    use db::schema::pais::dsl::*;
    let mut binding = state.posgresql.lock().unwrap();
    let connection = binding.deref_mut();
    let result = update(pais.filter(id_pais.eq(&new_country.country_id.trim())))
        .set((nombre_pais.eq(&new_country.country_name.trim()),))
        .execute(connection);
    match result {
        Ok(rows) => rows > 0,
        Err(_) => false,
    }
}

#[tauri::command(rename_all = "snake_case")]
fn delete_country(id: String, state: State<'_, AppState>) -> bool {
    use db::schema::pais::dsl::*;
    let mut binding = state.posgresql.lock().unwrap();
    let connection = binding.deref_mut();
    let result = delete(pais.filter(id_pais.eq(id.trim()))).execute(connection);
    match result {
        Ok(rows) => rows > 0,
        Err(_) => false,
    }
}

#[tauri::command(rename_all = "snake_case")]
fn create_country(new_country: Country, state: State<'_, AppState>) -> bool {
    use db::schema::pais::dsl::*;
    let mut binding = state.posgresql.lock().unwrap();
    let connection = binding.deref_mut();
    let result = insert_into(pais)
        .values((
            id_pais.eq(&new_country.country_id),
            nombre_pais.eq(&new_country.country_name),
        ))
        .execute(connection);
    match result {
        Ok(rows) => rows > 0,
        Err(_) => false,
    }
}

#[tauri::command(rename_all = "snake_case")]
fn get_authors(page: i64, state: State<'_, AppState>) -> Vec<Author> {
    use db::schema::autor::dsl::*;
    let offset = page * PAGE_SIZE;
    let result: Vec<Author> = {
        let mut binding = state.posgresql.lock().unwrap();
        info!("get_authors: page={}", page);
        let connection = binding.deref_mut();
        autor
            .offset(offset)
            .limit(PAGE_SIZE)
            .select(Author::as_select())
            .load(connection)
            .expect("Error loading authors")
    };
    result
}

#[tauri::command(rename_all = "snake_case")]
fn update_author(new_author: Author, state: State<'_, AppState>) -> bool {
    use db::schema::autor::dsl::*;
    let mut binding = state.posgresql.lock().unwrap();
    let connection = binding.deref_mut();
    let result = update(autor.filter(id_autor.eq(&new_author.author_id.trim())))
        .set((
            id_pais.eq(new_author.country_id.as_ref().map(|s| s.trim())),
            nombre_autor.eq(&new_author.author_name.trim()),
        ))
        .execute(connection);
    match result {
        Ok(rows) => rows > 0,
        Err(_) => false,
    }
}

#[tauri::command(rename_all = "snake_case")]
fn delete_author(id: String, state: State<'_, AppState>) -> bool {
    use db::schema::autor::dsl::*;
    let mut binding = state.posgresql.lock().unwrap();
    let connection = binding.deref_mut();
    let result = delete(autor.filter(id_autor.eq(id.trim()))).execute(connection);
    match result {
        Ok(rows) => rows > 0,
        Err(_) => false,
    }
}

#[tauri::command(rename_all = "snake_case")]
fn create_author(new_author: Author, state: State<'_, AppState>) -> bool {
    use db::schema::autor::dsl::*;
    let mut binding = state.posgresql.lock().unwrap();
    let connection = binding.deref_mut();
    let result = insert_into(autor)
        .values((
            id_autor.eq(&new_author.author_id),
            id_pais.eq(new_author.country_id.as_deref()),
            nombre_autor.eq(&new_author.author_name),
        ))
        .execute(connection);
    match result {
        Ok(rows) => rows > 0,
        Err(_) => false,
    }
}

#[tauri::command(rename_all = "snake_case")]
fn get_studios(page: i64, state: State<'_, AppState>) -> Vec<Studio> {
    use db::schema::estudio::dsl::*;
    let offset = page * PAGE_SIZE;
    let result: Vec<Studio> = {
        let mut binding = state.posgresql.lock().unwrap();
        info!("get_studios: page={}", page);
        let connection = binding.deref_mut();
        estudio
            .offset(offset)
            .limit(PAGE_SIZE)
            .select(Studio::as_select())
            .load(connection)
            .expect("Error loading studios")
    };
    result
}

#[tauri::command(rename_all = "snake_case")]
fn update_studio(new_studio: Studio, state: State<'_, AppState>) -> bool {
    use db::schema::estudio::dsl::*;
    let mut binding = state.posgresql.lock().unwrap();
    let connection = binding.deref_mut();
    let result = update(estudio.filter(id_estudio.eq(&new_studio.studio_id.trim())))
        .set((
            id_pais.eq(&new_studio.country_id.trim()),
            nombre_estudio.eq(&new_studio.studio_name.trim()),
        ))
        .execute(connection);
    match result {
        Ok(rows) => rows > 0,
        Err(_) => false,
    }
}

#[tauri::command(rename_all = "snake_case")]
fn delete_studio(id: String, state: State<'_, AppState>) -> bool {
    use db::schema::estudio::dsl::*;
    let mut binding = state.posgresql.lock().unwrap();
    let connection = binding.deref_mut();
    let result = delete(estudio.filter(id_estudio.eq(id.trim()))).execute(connection);
    match result {
        Ok(rows) => rows > 0,
        Err(_) => false,
    }
}

#[tauri::command(rename_all = "snake_case")]
fn create_studio(new_studio: Studio, state: State<'_, AppState>) -> bool {
    use db::schema::estudio::dsl::*;
    let mut binding = state.posgresql.lock().unwrap();
    let connection = binding.deref_mut();
    let result = insert_into(estudio)
        .values((
            id_estudio.eq(&new_studio.studio_id),
            id_pais.eq(&new_studio.country_id),
            nombre_estudio.eq(&new_studio.studio_name),
        ))
        .execute(connection);
    match result {
        Ok(rows) => rows > 0,
        Err(_) => false,
    }
}
#[tauri::command(rename_all = "snake_case")]
fn get_pegis(page: i64, state: State<'_, AppState>) -> Vec<Pegi> {
    use db::schema::pegi::dsl::*;
    let offset = page * PAGE_SIZE;
    let result: Vec<Pegi> = {
        let mut binding = state.posgresql.lock().unwrap();
        info!("get_pegis: page={}", page);
        let connection = binding.deref_mut();
        pegi.offset(offset)
            .limit(PAGE_SIZE)
            .select(Pegi::as_select())
            .load(connection)
            .expect("Error loading pegis")
    };
    result
}

#[tauri::command(rename_all = "snake_case")]
fn update_pegi(new_pegi: Pegi, state: State<'_, AppState>) -> bool {
    use db::schema::pegi::dsl::*;
    let mut binding = state.posgresql.lock().unwrap();
    let connection = binding.deref_mut();
    let result = update(pegi.filter(id_pegi.eq(&new_pegi.pegi_id.trim())))
        .set((categoria_pegi.eq(&new_pegi.pegi_category.trim()),))
        .execute(connection);
    match result {
        Ok(rows) => rows > 0,
        Err(_) => false,
    }
}

#[tauri::command(rename_all = "snake_case")]
fn delete_pegi(id: String, state: State<'_, AppState>) -> bool {
    use db::schema::pegi::dsl::*;
    let mut binding = state.posgresql.lock().unwrap();
    let connection = binding.deref_mut();
    let result = delete(pegi.filter(id_pegi.eq(id.trim()))).execute(connection);
    match result {
        Ok(rows) => rows > 0,
        Err(_) => false,
    }
}

#[tauri::command(rename_all = "snake_case")]
fn create_pegi(new_pegi: Pegi, state: State<'_, AppState>) -> bool {
    use db::schema::pegi::dsl::*;
    let mut binding = state.posgresql.lock().unwrap();
    let connection = binding.deref_mut();
    let result = insert_into(pegi)
        .values((
            id_pegi.eq(&new_pegi.pegi_id),
            categoria_pegi.eq(&new_pegi.pegi_category),
        ))
        .execute(connection);
    match result {
        Ok(rows) => rows > 0,
        Err(_) => false,
    }
}

#[tauri::command(rename_all = "snake_case")]
fn get_collections(page: i64, state: State<'_, AppState>) -> Vec<Collection> {
    use db::schema::coleccion::dsl::*;
    let offset = page * PAGE_SIZE;
    let result: Vec<Collection> = {
        let mut binding = state.posgresql.lock().unwrap();
        info!("get_collections: page={}", page);
        let connection = binding.deref_mut();
        coleccion
            .offset(offset)
            .limit(PAGE_SIZE)
            .select(Collection::as_select())
            .load(connection)
            .expect("Error loading collections")
    };
    result
}

#[tauri::command(rename_all = "snake_case")]
fn update_collection(new_collection: Collection, state: State<'_, AppState>) -> bool {
    use db::schema::coleccion::dsl::*;
    let mut binding = state.posgresql.lock().unwrap();
    let connection = binding.deref_mut();
    let result = update(coleccion.filter(id_coleccion.eq(&new_collection.collection_id.trim())))
        .set((
            nombre_coleccion.eq(&new_collection.collection_name.trim()),
            coleccion_padre.eq(new_collection.parent_collection.as_deref()),
        ))
        .execute(connection);
    match result {
        Ok(rows) => rows > 0,
        Err(_) => false,
    }
}

#[tauri::command(rename_all = "snake_case")]
fn delete_collection(id: String, state: State<'_, AppState>) -> bool {
    use db::schema::coleccion::dsl::*;
    let mut binding = state.posgresql.lock().unwrap();
    let connection = binding.deref_mut();
    let result = delete(coleccion.filter(id_coleccion.eq(id.trim()))).execute(connection);
    match result {
        Ok(rows) => rows > 0,
        Err(_) => false,
    }
}

#[tauri::command(rename_all = "snake_case")]
fn create_collection(new_collection: Collection, state: State<'_, AppState>) -> bool {
    use db::schema::coleccion::dsl::*;
    let mut binding = state.posgresql.lock().unwrap();
    let connection = binding.deref_mut();
    let result = insert_into(coleccion)
        .values((
            id_coleccion.eq(&new_collection.collection_id),
            nombre_coleccion.eq(&new_collection.collection_name),
            coleccion_padre.eq(new_collection.parent_collection.as_deref()),
        ))
        .execute(connection);
    match result {
        Ok(rows) => rows > 0,
        Err(_) => false,
    }
}

#[tauri::command(rename_all = "snake_case")]
fn get_videogames_and_studios(page: i64, state: State<'_, AppState>) -> Vec<(i64, String)> {
    use db::schema::estudio_de_un_videojuego;
    use db::schema::videojuego;
    let offset = page * PAGE_SIZE;
    let result: Vec<(i64, String)> = {
        let mut binding = state.posgresql.lock().unwrap();
        info!("get_collections: page={}", page);
        let connection = binding.deref_mut();
        estudio_de_un_videojuego::table
            .inner_join(
                videojuego::table
                    .on(estudio_de_un_videojuego::id_videojuego.eq(videojuego::id_objeto)),
            )
            .group_by(videojuego::nombre_objeto)
            .select((
                count(estudio_de_un_videojuego::id_estudio),
                videojuego::nombre_objeto,
            ))
            .offset(offset)
            .limit(PAGE_SIZE)
            .load::<(i64, String)>(connection)
            .unwrap_or_default()
    };
    result
}

#[tauri::command(rename_all = "snake_case")]
fn get_videogames_and_studios_by_name(
    page: i64,
    name: String,
    state: State<'_, AppState>,
) -> Vec<(i64, String)> {
    use db::schema::estudio_de_un_videojuego;
    use db::schema::videojuego;
    let offset = page * PAGE_SIZE;
    let result: Vec<(i64, String)> = {
        let mut binding = state.posgresql.lock().unwrap();
        info!("get_collections: page={}", page);
        let connection = binding.deref_mut();
        estudio_de_un_videojuego::table
            .inner_join(
                videojuego::table
                    .on(estudio_de_un_videojuego::id_videojuego.eq(videojuego::id_objeto)),
            )
            .group_by(videojuego::nombre_objeto)
            .select((
                count(estudio_de_un_videojuego::id_estudio),
                videojuego::nombre_objeto,
            ))
            .filter(videojuego::nombre_objeto.ilike(format!("%{}%", name.trim())))
            .offset(offset)
            .limit(PAGE_SIZE)
            .load::<(i64, String)>(connection)
            .unwrap_or_default()
    };
    result
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
            get_book_by_name,
            get_music_by_name,
            get_videogame_by_name,
            update_book,
            update_music,
            update_videogame,
            delete_book,
            delete_music,
            delete_videogame,
            create_book,
            create_music,
            create_videogame,
            get_identifiers,
            update_identifier,
            delete_identifier,
            create_identifier,
            get_languages,
            update_language,
            delete_language,
            create_language,
            get_countries,
            update_country,
            delete_country,
            create_country,
            get_authors,
            update_author,
            delete_author,
            create_author,
            get_studios,
            update_studio,
            delete_studio,
            create_studio,
            get_pegis,
            update_pegi,
            delete_pegi,
            create_pegi,
            get_collections,
            update_collection,
            delete_collection,
            create_collection,
            get_distributors,
            update_distributor,
            delete_distributor,
            create_distributor,
            get_genres,
            update_genre,
            delete_genre,
            create_genre,
            get_videogames_and_studios,
            get_videogames_and_studios_by_name,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
