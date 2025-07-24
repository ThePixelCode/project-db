/*==============================================================*/
/* Table: AUTOR                                                 */
/*==============================================================*/
create table AUTOR (
   ID_AUTOR             CHAR(256)            not null,
   ID_PAIS              CHAR(256)            null,
   NOMBRE_AUTOR         CHAR(256)            not null,
   constraint PK_AUTOR primary key (ID_AUTOR)
);

/*==============================================================*/
/* Index: AUTOR_PK                                              */
/*==============================================================*/
create unique index AUTOR_PK on AUTOR (
ID_AUTOR
);

/*==============================================================*/
/* Index: PAIS_DEL_AUTOR_FK                                     */
/*==============================================================*/
create  index PAIS_DEL_AUTOR_FK on AUTOR (
ID_PAIS
);

/*==============================================================*/
/* Table: AUTOR_DEL_OBJETO                                      */
/*==============================================================*/
create table AUTOR_DEL_OBJETO (
   ID_OBJETO            CHAR(256)            not null,
   ID_AUTOR             CHAR(256)            not null,
   TIPO_DE_OBJETO       INT4                 not null,
   constraint PK_AUTOR_DEL_OBJETO primary key (ID_OBJETO, ID_AUTOR)
);

/*==============================================================*/
/* Index: AUTOR_DEL_OBJETO_PK                                   */
/*==============================================================*/
create unique index AUTOR_DEL_OBJETO_PK on AUTOR_DEL_OBJETO (
ID_OBJETO,
ID_AUTOR
);

/*==============================================================*/
/* Index: AUTOR_DEL_OBJETO2_FK                                  */
/*==============================================================*/
create  index AUTOR_DEL_OBJETO2_FK on AUTOR_DEL_OBJETO (
ID_AUTOR
);

/*==============================================================*/
/* Table: COLECCION                                             */
/*==============================================================*/
create table COLECCION (
   ID_COLECCION         CHAR(256)            not null,
   NOMBRE_COLECCION     CHAR(256)            not null,
   COLECCION_PADRE      CHAR(256)            null,
   constraint PK_COLECCION primary key (ID_COLECCION)
);

/*==============================================================*/
/* Index: COLECCION_PK                                          */
/*==============================================================*/
create unique index COLECCION_PK on COLECCION (
ID_COLECCION
);

/*==============================================================*/
/* Index: SUB_COLECCIONES_FK                                    */
/*==============================================================*/
create  index SUB_COLECCIONES_FK on COLECCION (
COLECCION_PADRE
);

/*==============================================================*/
/* Table: DISTRIBUIDOR                                          */
/*==============================================================*/
create table DISTRIBUIDOR (
   ID_DISTRIBUIDOR      CHAR(256)            not null,
   ID_PAIS              CHAR(256)            not null,
   NOMBRE_DISTRIBUIDOR  CHAR(256)            not null,
   ANO_FUNDACION        DATE                 not null,
   constraint PK_DISTRIBUIDOR primary key (ID_DISTRIBUIDOR)
);

/*==============================================================*/
/* Index: DISTRIBUIDOR_PK                                       */
/*==============================================================*/
create unique index DISTRIBUIDOR_PK on DISTRIBUIDOR (
ID_DISTRIBUIDOR
);

/*==============================================================*/
/* Index: PAIS_DEL_DISTRIBUIDOR_FK                              */
/*==============================================================*/
create  index PAIS_DEL_DISTRIBUIDOR_FK on DISTRIBUIDOR (
ID_PAIS
);

/*==============================================================*/
/* Table: DISTRIBUIDOR_DEL_OBJETO                               */
/*==============================================================*/
create table DISTRIBUIDOR_DEL_OBJETO (
   ID_OBJETO            CHAR(256)            not null,
   ID_DISTRIBUIDOR      CHAR(256)            not null,
   TIPO_DE_OBJETO       INT4                 not null,
   constraint PK_DISTRIBUIDOR_DEL_OBJETO primary key (ID_OBJETO, ID_DISTRIBUIDOR)
);

/*==============================================================*/
/* Index: DISTRIBUIDOR_DEL_OBJETO_PK                            */
/*==============================================================*/
create unique index DISTRIBUIDOR_DEL_OBJETO_PK on DISTRIBUIDOR_DEL_OBJETO (
ID_OBJETO,
ID_DISTRIBUIDOR
);

/*==============================================================*/
/* Index: DISTRIBUIDOR_DEL_OBJETO2_FK                           */
/*==============================================================*/
create  index DISTRIBUIDOR_DEL_OBJETO2_FK on DISTRIBUIDOR_DEL_OBJETO (
ID_DISTRIBUIDOR
);

/*==============================================================*/
/* Table: ESTUDIO                                               */
/*==============================================================*/
create table ESTUDIO (
   ID_ESTUDIO           CHAR(256)            not null,
   ID_PAIS              CHAR(256)            not null,
   NOMBRE_ESTUDIO       CHAR(256)            not null,
   constraint PK_ESTUDIO primary key (ID_ESTUDIO)
);

/*==============================================================*/
/* Index: ESTUDIO_PK                                            */
/*==============================================================*/
create unique index ESTUDIO_PK on ESTUDIO (
ID_ESTUDIO
);

/*==============================================================*/
/* Index: PAIS_DEL_ESTUDIO_FK                                   */
/*==============================================================*/
create  index PAIS_DEL_ESTUDIO_FK on ESTUDIO (
ID_PAIS
);

/*==============================================================*/
/* Table: ESTUDIO_DE_UN_VIDEOJUEGO                              */
/*==============================================================*/
create table ESTUDIO_DE_UN_VIDEOJUEGO (
   ID_ESTUDIO           CHAR(256)            not null,
   ID_VIDEOJUEGO        CHAR(256)            not null,
   constraint PK_ESTUDIO_DE_UN_VIDEOJUEGO primary key (ID_ESTUDIO, ID_VIDEOJUEGO)
);

/*==============================================================*/
/* Index: ESTUDIO_DE_UN_VIDEOJUEGO2_FK                          */
/*==============================================================*/
create  index ESTUDIO_DE_UN_VIDEOJUEGO2_FK on ESTUDIO_DE_UN_VIDEOJUEGO (
ID_ESTUDIO
);

/*==============================================================*/
/* Index: ESTUDIO_DE_UN_VIDEOJUEGO_FK                           */
/*==============================================================*/
create  index ESTUDIO_DE_UN_VIDEOJUEGO_FK on ESTUDIO_DE_UN_VIDEOJUEGO (
ID_VIDEOJUEGO
);

/*==============================================================*/
/* Table: GENERO                                                */
/*==============================================================*/
create table GENERO (
   ID_GENERO            CHAR(256)            not null,
   NOMBRE_GENERO        CHAR(256)            not null,
   constraint PK_GENERO primary key (ID_GENERO)
);

/*==============================================================*/
/* Index: GENERO_PK                                             */
/*==============================================================*/
create unique index GENERO_PK on GENERO (
ID_GENERO
);

/*==============================================================*/
/* Table: GENERO_DEL_OBJETO                                     */
/*==============================================================*/
create table GENERO_DEL_OBJETO (
   ID_OBJETO            CHAR(256)            not null,
   ID_GENERO            CHAR(256)            not null,
   TIPO_DE_OBJETO       INT4                 not null,
   constraint PK_GENERO_DEL_OBJETO primary key (ID_OBJETO, ID_GENERO)
);

/*==============================================================*/
/* Index: GENERO_DEL_OBJETO_PK                                  */
/*==============================================================*/
create unique index GENERO_DEL_OBJETO_PK on GENERO_DEL_OBJETO (
ID_OBJETO,
ID_GENERO
);

/*==============================================================*/
/* Index: GENERO_DEL_OBJETO2_FK                                 */
/*==============================================================*/
create  index GENERO_DEL_OBJETO2_FK on GENERO_DEL_OBJETO (
ID_GENERO
);

/*==============================================================*/
/* Table: IDENTIFICADOR                                         */
/*==============================================================*/
create table IDENTIFICADOR (
   ID_IDENTIFICADOR     CHAR(256)            not null,
   IDENTIFICADOR_DEWEY_DE_OBJETO CHAR(256)            not null,
   constraint PK_IDENTIFICADOR primary key (ID_IDENTIFICADOR)
);

/*==============================================================*/
/* Index: IDENTIFICADOR_PK                                      */
/*==============================================================*/
create unique index IDENTIFICADOR_PK on IDENTIFICADOR (
ID_IDENTIFICADOR
);

/*==============================================================*/
/* Table: IDIOMA                                                */
/*==============================================================*/
create table IDIOMA (
   ID_IDIOMA            CHAR(256)            not null,
   NOMBRE_IDIOMA        CHAR(256)            not null,
   constraint PK_IDIOMA primary key (ID_IDIOMA)
);

/*==============================================================*/
/* Index: IDIOMA_PK                                             */
/*==============================================================*/
create unique index IDIOMA_PK on IDIOMA (
ID_IDIOMA
);

/*==============================================================*/
/* Table: LIBRO                                                 */
/*==============================================================*/
create table LIBRO (
   ID_OBJETO            CHAR(256)            not null,
   NOMBRE_OBJETO        CHAR(256)            not null,
   ANO_PUBLICACION      DATE                 null,
   ID_IDENTIFICADOR     CHAR(256)            not null,
   ID_IDIOMA            CHAR(256)            not null,
   CANTIDAD_DE_PAGINAS  INT4                 not null,
   constraint PK_LIBRO primary key (ID_OBJETO)
);

/*==============================================================*/
/* Index: IDENTIFICADOR_DEL_OBJETO2_FK                          */
/*==============================================================*/
create  index IDENTIFICADOR_DEL_OBJETO2_FK on LIBRO (
ID_IDENTIFICADOR
);

/*==============================================================*/
/* Index: IDIOMA_DEL_OBJETO2_FK                                 */
/*==============================================================*/
create  index IDIOMA_DEL_OBJETO2_FK on LIBRO (
ID_IDIOMA
);

/*==============================================================*/
/* Table: MUSICA                                                */
/*==============================================================*/
create table MUSICA (
   ID_OBJETO            CHAR(256)            not null,
   NOMBRE_OBJETO        CHAR(256)            not null,
   ANO_PUBLICACION      DATE                 null,
   ID_IDENTIFICADOR     CHAR(256)            not null,
   ID_IDIOMA            CHAR(256)            not null,
   DURACION             TIME                 not null,
   constraint PK_MUSICA primary key (ID_OBJETO)
);

/*==============================================================*/
/* Index: IDENTIFICADOR_DEL_OBJETO_FK                           */
/*==============================================================*/
create  index IDENTIFICADOR_DEL_OBJETO_FK on MUSICA (
ID_IDENTIFICADOR
);

/*==============================================================*/
/* Index: IDIOMA_DEL_OBJETO_FK                                  */
/*==============================================================*/
create  index IDIOMA_DEL_OBJETO_FK on MUSICA (
ID_IDIOMA
);

/*==============================================================*/
/* Table: OBJETO_EN_COLECCION                                   */
/*==============================================================*/
create table OBJETO_EN_COLECCION (
   ID_OBJETO            CHAR(256)            not null,
   ID_COLECCION         CHAR(256)            not null,
   TIPO_DE_OBJETO       INT4                 not null,
   constraint PK_OBJETO_EN_COLECCION primary key (ID_OBJETO, ID_COLECCION)
);

/*==============================================================*/
/* Index: OBJETO_EN_COLECCION_PK                                */
/*==============================================================*/
create unique index OBJETO_EN_COLECCION_PK on OBJETO_EN_COLECCION (
ID_OBJETO,
ID_COLECCION
);

/*==============================================================*/
/* Index: OBJETO_EN_COLECCION2_FK                               */
/*==============================================================*/
create  index OBJETO_EN_COLECCION2_FK on OBJETO_EN_COLECCION (
ID_COLECCION
);

/*==============================================================*/
/* Table: PAIS                                                  */
/*==============================================================*/
create table PAIS (
   ID_PAIS              CHAR(256)            not null,
   NOMBRE_PAIS          CHAR(256)            not null,
   constraint PK_PAIS primary key (ID_PAIS)
);

/*==============================================================*/
/* Index: PAIS_PK                                               */
/*==============================================================*/
create unique index PAIS_PK on PAIS (
ID_PAIS
);

/*==============================================================*/
/* Table: PEGI                                                  */
/*==============================================================*/
create table PEGI (
   ID_PEGI              CHAR(256)            not null,
   CATEGORIA_PEGI       CHAR(256)            not null,
   constraint PK_PEGI primary key (ID_PEGI)
);

/*==============================================================*/
/* Index: PEGI_PK                                               */
/*==============================================================*/
create unique index PEGI_PK on PEGI (
ID_PEGI
);

/*==============================================================*/
/* Table: VIDEOJUEGO                                            */
/*==============================================================*/
create table VIDEOJUEGO (
   ID_OBJETO            CHAR(256)            not null,
   NOMBRE_OBJETO        CHAR(256)            not null,
   ANO_PUBLICACION      DATE                 null,
   ID_IDENTIFICADOR     CHAR(256)            not null,
   ID_IDIOMA            CHAR(256)            not null,
   SOPORTE_DE_MANDO     BOOL                 not null,
   ID_PEGI              CHAR(256)            not null,
   constraint PK_VIDEOJUEGO primary key (ID_OBJETO)
);

/*==============================================================*/
/* Index: PEGUI_DEL_JUEGO_FK                                    */
/*==============================================================*/
create  index PEGUI_DEL_JUEGO_FK on VIDEOJUEGO (
ID_PEGI
);

/*==============================================================*/
/* Index: IDENTIFICADOR_DEL_OBJETO3_FK                          */
/*==============================================================*/
create  index IDENTIFICADOR_DEL_OBJETO3_FK on VIDEOJUEGO (
ID_IDENTIFICADOR
);

/*==============================================================*/
/* Index: IDIOMA_DEL_OBJETO3_FK                                 */
/*==============================================================*/
create  index IDIOMA_DEL_OBJETO3_FK on VIDEOJUEGO (
ID_IDIOMA
);

alter table AUTOR
   add constraint FK_AUTOR_PAIS_DEL__PAIS foreign key (ID_PAIS)
      references PAIS (ID_PAIS)
      on delete restrict on update restrict;

alter table AUTOR_DEL_OBJETO
   add constraint FK_AUTOR_DE_AUTOR_DEL_AUTOR foreign key (ID_AUTOR)
      references AUTOR (ID_AUTOR)
      on delete restrict on update restrict;

alter table COLECCION
   add constraint FK_COLECCIO_SUB_COLEC_COLECCIO foreign key (COLECCION_PADRE)
      references COLECCION (ID_COLECCION)
      on delete restrict on update restrict;

alter table DISTRIBUIDOR
   add constraint FK_DISTRIBU_PAIS_DEL__PAIS foreign key (ID_PAIS)
      references PAIS (ID_PAIS)
      on delete restrict on update restrict;

alter table DISTRIBUIDOR_DEL_OBJETO
   add constraint FK_DISTRIBU_DISTRIBUI_DISTRIBU foreign key (ID_DISTRIBUIDOR)
      references DISTRIBUIDOR (ID_DISTRIBUIDOR)
      on delete restrict on update restrict;

alter table ESTUDIO
   add constraint FK_ESTUDIO_PAIS_DEL__PAIS foreign key (ID_PAIS)
      references PAIS (ID_PAIS)
      on delete restrict on update restrict;

alter table ESTUDIO_DE_UN_VIDEOJUEGO
   add constraint FK_ESTUDIO__ESTUDIO_D_VIDEOJUE foreign key (ID_VIDEOJUEGO)
      references VIDEOJUEGO (ID_OBJETO)
      on delete restrict on update restrict;

alter table ESTUDIO_DE_UN_VIDEOJUEGO
   add constraint FK_ESTUDIO__ESTUDIO_D_ESTUDIO foreign key (ID_ESTUDIO)
      references ESTUDIO (ID_ESTUDIO)
      on delete restrict on update restrict;

alter table GENERO_DEL_OBJETO
   add constraint FK_GENERO_D_GENERO_DE_GENERO foreign key (ID_GENERO)
      references GENERO (ID_GENERO)
      on delete restrict on update restrict;

alter table LIBRO
   add constraint FK_LIBRO_IDENTIFIC_IDENTIFI foreign key (ID_IDENTIFICADOR)
      references IDENTIFICADOR (ID_IDENTIFICADOR)
      on delete restrict on update restrict;

alter table LIBRO
   add constraint FK_LIBRO_IDIOMA_DE_IDIOMA foreign key (ID_IDIOMA)
      references IDIOMA (ID_IDIOMA)
      on delete restrict on update restrict;

alter table MUSICA
   add constraint FK_MUSICA_IDENTIFIC_IDENTIFI foreign key (ID_IDENTIFICADOR)
      references IDENTIFICADOR (ID_IDENTIFICADOR)
      on delete restrict on update restrict;

alter table MUSICA
   add constraint FK_MUSICA_IDIOMA_DE_IDIOMA foreign key (ID_IDIOMA)
      references IDIOMA (ID_IDIOMA)
      on delete restrict on update restrict;

alter table OBJETO_EN_COLECCION
   add constraint FK_OBJETO_E_OBJETO_EN_COLECCIO foreign key (ID_COLECCION)
      references COLECCION (ID_COLECCION)
      on delete restrict on update restrict;

alter table VIDEOJUEGO
   add constraint FK_VIDEOJUE_IDENTIFIC_IDENTIFI foreign key (ID_IDENTIFICADOR)
      references IDENTIFICADOR (ID_IDENTIFICADOR)
      on delete restrict on update restrict;

alter table VIDEOJUEGO
   add constraint FK_VIDEOJUE_IDIOMA_DE_IDIOMA foreign key (ID_IDIOMA)
      references IDIOMA (ID_IDIOMA)
      on delete restrict on update restrict;

alter table VIDEOJUEGO
   add constraint FK_VIDEOJUE_PEGUI_DEL_PEGI foreign key (ID_PEGI)
      references PEGI (ID_PEGI)
      on delete restrict on update restrict;

