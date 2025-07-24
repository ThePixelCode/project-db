/*==============================================================*/
/* DBMS name:      PostgreSQL 9.x                               */
/* Created on:     24/7/2025 19:05:38                           */
/*==============================================================*/


drop index if exists PAIS_DEL_AUTOR_FK cascade;

drop index if exists AUTOR_PK cascade;

drop table if exists AUTOR cascade;

drop index if exists AUTOR_DEL_OBJETO2_FK cascade;

drop index if exists AUTOR_DEL_OBJETO_PK cascade;

drop table if exists AUTOR_DEL_OBJETO cascade;

drop index if exists SUB_COLECCIONES_FK cascade;

drop index if exists COLECCION_PK cascade;

drop table if exists COLECCION cascade;

drop index if exists PAIS_DEL_DISTRIBUIDOR_FK cascade;

drop index if exists DISTRIBUIDOR_PK cascade;

drop table if exists DISTRIBUIDOR cascade;

drop index if exists DISTRIBUIDOR_DEL_OBJETO2_FK cascade;

drop index if exists DISTRIBUIDOR_DEL_OBJETO_PK cascade;

drop table if exists DISTRIBUIDOR_DEL_OBJETO cascade;

drop index if exists PAIS_DEL_ESTUDIO_FK cascade;

drop index if exists ESTUDIO_PK cascade;

drop table if exists ESTUDIO cascade;

drop index if exists ESTUDIO_DE_UN_VIDEOJUEGO_FK cascade;

drop index if exists ESTUDIO_DE_UN_VIDEOJUEGO2_FK cascade;

drop table if exists ESTUDIO_DE_UN_VIDEOJUEGO cascade;

drop index if exists GENERO_PK cascade;

drop table if exists GENERO cascade;

drop index if exists GENERO_DEL_OBJETO2_FK cascade;

drop index if exists GENERO_DEL_OBJETO_PK cascade;

drop table if exists GENERO_DEL_OBJETO cascade;

drop index if exists IDENTIFICADOR_PK cascade;

drop table if exists IDENTIFICADOR cascade;

drop index if exists IDIOMA_PK cascade;

drop table if exists IDIOMA cascade;

drop index if exists IDIOMA_DEL_OBJETO2_FK cascade;

drop index if exists IDENTIFICADOR_DEL_OBJETO2_FK cascade;

drop table if exists LIBRO cascade;

drop index if exists IDIOMA_DEL_OBJETO_FK cascade;

drop index if exists IDENTIFICADOR_DEL_OBJETO_FK cascade;

drop table if exists MUSICA cascade;

drop index if exists OBJETO_EN_COLECCION2_FK cascade;

drop index if exists OBJETO_EN_COLECCION_PK cascade;

drop table if exists OBJETO_EN_COLECCION cascade;

drop index if exists PAIS_PK cascade;

drop table if exists PAIS cascade;

drop index if exists PEGI_PK cascade;

drop table if exists PEGI cascade;

drop index if exists IDIOMA_DEL_OBJETO3_FK cascade;

drop index if exists IDENTIFICADOR_DEL_OBJETO3_FK cascade;

drop index if exists PEGUI_DEL_JUEGO_FK cascade;

drop table if exists VIDEOJUEGO cascade;