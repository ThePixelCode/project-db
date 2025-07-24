use super::schema;
use chrono::{NaiveDate, NaiveTime};
use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::autor)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Author {
    #[diesel(column_name = "id_autor")]
    pub author_id: String,
    #[diesel(column_name = "id_pais")]
    pub country_id: Option<String>,
    #[diesel(column_name = "nombre_autor")]
    pub author_name: String,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(primary_key(id_objeto, id_autor))]
#[diesel(table_name = schema::autor_del_objeto)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AuthorOfObject {
    #[diesel(column_name = "id_objeto")]
    pub object_id: String,
    #[diesel(column_name = "id_autor")]
    pub author_id: String,
    #[diesel(column_name = "tipo_de_objeto")]
    pub object_type: i32,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::coleccion)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Collection {
    #[diesel(column_name = "id_coleccion")]
    pub collection_id: String,
    #[diesel(column_name = "nombre_coleccion")]
    pub collection_name: String,
    #[diesel(column_name = "coleccion_padre")]
    pub parent_collection: Option<String>,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::distribuidor)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Distributor {
    #[diesel(column_name = "id_distribuidor")]
    pub distributor_id: String,
    #[diesel(column_name = "id_pais")]
    pub country_id: String,
    #[diesel(column_name = "nombre_distribuidor")]
    pub distributor_name: String,
    #[diesel(column_name = "ano_fundacion")]
    pub foundation_year: NaiveDate,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(primary_key(id_objeto, id_distribuidor))]
#[diesel(table_name = schema::distribuidor_del_objeto)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DistributorOfObject {
    #[diesel(column_name = "id_objeto")]
    pub object_id: String,
    #[diesel(column_name = "id_distribuidor")]
    pub distributor_id: String,
    #[diesel(column_name = "tipo_de_objeto")]
    pub object_type: i32,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::estudio)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Studio {
    #[diesel(column_name = "id_estudio")]
    pub studio_id: String,
    #[diesel(column_name = "id_pais")]
    pub country_id: String,
    #[diesel(column_name = "nombre_estudio")]
    pub studio_name: String,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(primary_key(id_estudio, id_videojuego))]
#[diesel(table_name = schema::estudio_de_un_videojuego)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StudioOfVideogame {
    #[diesel(column_name = "id_estudio")]
    pub studio_id: String,
    #[diesel(column_name = "id_videojuego")]
    pub videogame_id: String,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::genero)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Genre {
    #[diesel(column_name = "id_genero")]
    pub genre_id: String,
    #[diesel(column_name = "nombre_genero")]
    pub genre_name: String,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(primary_key(id_objeto, id_genero))]
#[diesel(table_name = schema::genero_del_objeto)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GenreOfObject {
    #[diesel(column_name = "id_objeto")]
    pub object_id: String,
    #[diesel(column_name = "id_genero")]
    pub genre_id: String,
    #[diesel(column_name = "tipo_de_objeto")]
    pub object_type: i32,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::identificador)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Identifier {
    #[diesel(column_name = "id_identificador")]
    pub identifier_id: String,
    #[diesel(column_name = "identificador_dewey_de_objeto")]
    pub dewey_identifier: String,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::idioma)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Language {
    #[diesel(column_name = "id_idioma")]
    pub language_id: String,
    #[diesel(column_name = "nombre_idioma")]
    pub language_name: String,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::libro)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Book {
    #[diesel(column_name = "id_objeto")]
    pub object_id: String,
    #[diesel(column_name = "nombre_objeto")]
    pub object_name: String,
    #[diesel(column_name = "ano_publicacion")]
    pub publication_year: Option<NaiveDate>,
    #[diesel(column_name = "id_identificador")]
    pub identifier_id: String,
    #[diesel(column_name = "id_idioma")]
    pub language_id: String,
    #[diesel(column_name = "cantidad_de_paginas")]
    pub page_count: i32,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::musica)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Music {
    #[diesel(column_name = "id_objeto")]
    pub object_id: String,
    #[diesel(column_name = "nombre_objeto")]
    pub object_name: String,
    #[diesel(column_name = "ano_publicacion")]
    pub publication_year: Option<NaiveDate>,
    #[diesel(column_name = "id_identificador")]
    pub identifier_id: String,
    #[diesel(column_name = "id_idioma")]
    pub language_id: String,
    #[diesel(column_name = "duracion")]
    pub duration: NaiveTime,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(primary_key(id_objeto, id_coleccion))]
#[diesel(table_name = schema::objeto_en_coleccion)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ObjectInCollection {
    #[diesel(column_name = "id_objeto")]
    pub object_id: String,
    #[diesel(column_name = "id_coleccion")]
    pub collection_id: String,
    #[diesel(column_name = "tipo_de_objeto")]
    pub object_type: i32,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::pais)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Country {
    #[diesel(column_name = "id_pais")]
    pub country_id: String,
    #[diesel(column_name = "nombre_pais")]
    pub country_name: String,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::pegi)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Pegi {
    #[diesel(column_name = "id_pegi")]
    pub pegi_id: String,
    #[diesel(column_name = "categoria_pegi")]
    pub pegi_category: String,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::videojuego)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Videogame {
    #[diesel(column_name = "id_objeto")]
    pub object_id: String,
    #[diesel(column_name = "nombre_objeto")]
    pub object_name: String,
    #[diesel(column_name = "ano_publicacion")]
    pub publication_year: Option<NaiveDate>,
    #[diesel(column_name = "id_identificador")]
    pub identifier_id: String,
    #[diesel(column_name = "id_idioma")]
    pub language_id: String,
    #[diesel(column_name = "soporte_de_mando")]
    pub controller_support: bool,
    #[diesel(column_name = "id_pegi")]
    pub pegi_id: String,
}
