// @generated automatically by Diesel CLI.

diesel::table! {
    autor (id_autor) {
        #[max_length = 256]
        id_autor -> Bpchar,
        #[max_length = 256]
        id_pais -> Nullable<Bpchar>,
        #[max_length = 256]
        nombre_autor -> Bpchar,
    }
}

diesel::table! {
    autor_del_objeto (id_objeto, id_autor) {
        #[max_length = 256]
        id_objeto -> Bpchar,
        #[max_length = 256]
        id_autor -> Bpchar,
        tipo_de_objeto -> Int4,
    }
}

diesel::table! {
    coleccion (id_coleccion) {
        #[max_length = 256]
        id_coleccion -> Bpchar,
        #[max_length = 256]
        nombre_coleccion -> Bpchar,
        #[max_length = 256]
        coleccion_padre -> Nullable<Bpchar>,
    }
}

diesel::table! {
    distribuidor (id_distribuidor) {
        #[max_length = 256]
        id_distribuidor -> Bpchar,
        #[max_length = 256]
        id_pais -> Bpchar,
        #[max_length = 256]
        nombre_distribuidor -> Bpchar,
        ano_fundacion -> Date,
    }
}

diesel::table! {
    distribuidor_del_objeto (id_objeto, id_distribuidor) {
        #[max_length = 256]
        id_objeto -> Bpchar,
        #[max_length = 256]
        id_distribuidor -> Bpchar,
        tipo_de_objeto -> Int4,
    }
}

diesel::table! {
    estudio (id_estudio) {
        #[max_length = 256]
        id_estudio -> Bpchar,
        #[max_length = 256]
        id_pais -> Bpchar,
        #[max_length = 256]
        nombre_estudio -> Bpchar,
    }
}

diesel::table! {
    estudio_de_un_videojuego (id_estudio, id_videojuego) {
        #[max_length = 256]
        id_estudio -> Bpchar,
        #[max_length = 256]
        id_videojuego -> Bpchar,
    }
}

diesel::table! {
    genero (id_genero) {
        #[max_length = 256]
        id_genero -> Bpchar,
        #[max_length = 256]
        nombre_genero -> Bpchar,
    }
}

diesel::table! {
    genero_del_objeto (id_objeto, id_genero) {
        #[max_length = 256]
        id_objeto -> Bpchar,
        #[max_length = 256]
        id_genero -> Bpchar,
        tipo_de_objeto -> Int4,
    }
}

diesel::table! {
    identificador (id_identificador) {
        #[max_length = 256]
        id_identificador -> Bpchar,
        #[max_length = 256]
        identificador_dewey_de_objeto -> Bpchar,
    }
}

diesel::table! {
    idioma (id_idioma) {
        #[max_length = 256]
        id_idioma -> Bpchar,
        #[max_length = 256]
        nombre_idioma -> Bpchar,
    }
}

diesel::table! {
    libro (id_objeto) {
        #[max_length = 256]
        id_objeto -> Bpchar,
        #[max_length = 256]
        nombre_objeto -> Bpchar,
        ano_publicacion -> Nullable<Date>,
        #[max_length = 256]
        id_identificador -> Bpchar,
        #[max_length = 256]
        id_idioma -> Bpchar,
        cantidad_de_paginas -> Int4,
    }
}

diesel::table! {
    musica (id_objeto) {
        #[max_length = 256]
        id_objeto -> Bpchar,
        #[max_length = 256]
        nombre_objeto -> Bpchar,
        ano_publicacion -> Nullable<Date>,
        #[max_length = 256]
        id_identificador -> Bpchar,
        #[max_length = 256]
        id_idioma -> Bpchar,
        duracion -> Time,
    }
}

diesel::table! {
    objeto_en_coleccion (id_objeto, id_coleccion) {
        #[max_length = 256]
        id_objeto -> Bpchar,
        #[max_length = 256]
        id_coleccion -> Bpchar,
        tipo_de_objeto -> Int4,
    }
}

diesel::table! {
    pais (id_pais) {
        #[max_length = 256]
        id_pais -> Bpchar,
        #[max_length = 256]
        nombre_pais -> Bpchar,
    }
}

diesel::table! {
    pegi (id_pegi) {
        #[max_length = 256]
        id_pegi -> Bpchar,
        #[max_length = 256]
        categoria_pegi -> Bpchar,
    }
}

diesel::table! {
    videojuego (id_objeto) {
        #[max_length = 256]
        id_objeto -> Bpchar,
        #[max_length = 256]
        nombre_objeto -> Bpchar,
        ano_publicacion -> Nullable<Date>,
        #[max_length = 256]
        id_identificador -> Bpchar,
        #[max_length = 256]
        id_idioma -> Bpchar,
        soporte_de_mando -> Bool,
        #[max_length = 256]
        id_pegi -> Bpchar,
    }
}

diesel::joinable!(autor -> pais (id_pais));
diesel::joinable!(autor_del_objeto -> autor (id_autor));
diesel::joinable!(distribuidor -> pais (id_pais));
diesel::joinable!(distribuidor_del_objeto -> distribuidor (id_distribuidor));
diesel::joinable!(estudio -> pais (id_pais));
diesel::joinable!(estudio_de_un_videojuego -> estudio (id_estudio));
diesel::joinable!(estudio_de_un_videojuego -> videojuego (id_videojuego));
diesel::joinable!(genero_del_objeto -> genero (id_genero));
diesel::joinable!(libro -> identificador (id_identificador));
diesel::joinable!(libro -> idioma (id_idioma));
diesel::joinable!(musica -> identificador (id_identificador));
diesel::joinable!(musica -> idioma (id_idioma));
diesel::joinable!(objeto_en_coleccion -> coleccion (id_coleccion));
diesel::joinable!(videojuego -> identificador (id_identificador));
diesel::joinable!(videojuego -> idioma (id_idioma));
diesel::joinable!(videojuego -> pegi (id_pegi));

diesel::allow_tables_to_appear_in_same_query!(
    autor,
    autor_del_objeto,
    coleccion,
    distribuidor,
    distribuidor_del_objeto,
    estudio,
    estudio_de_un_videojuego,
    genero,
    genero_del_objeto,
    identificador,
    idioma,
    libro,
    musica,
    objeto_en_coleccion,
    pais,
    pegi,
    videojuego,
);
