export type Author = {
    author_id: string;
    country_id?: string;
    author_name: string;
};

export type AuthorOfObject = {
    object_id: string;
    author_id: string;
    object_type: number;
};

export type Collection = {
    collection_id: string;
    collection_name: string;
    parent_collection?: string;
};

export type Distributor = {
    distributor_id: string;
    country_id: string;
    distributor_name: string;
    foundation_year: string;
};

export type DistributorOfObject = {
    object_id: string;
    distributor_id: string;
    object_type: number;
};

export type Studio = {
    studio_id: string;
    country_id: string;
    studio_name: string;
};

export type StudioOfVideogame = {
    studio_id: string;
    videogame_id: string;
};

export type Genre = {
    genre_id: string;
    genre_name: string;
};

export type GenreOfObject = {
    object_id: string;
    genre_id: string;
    object_type: number;
};

export type Identifier = {
    identifier_id: string;
    dewey_identifier: string;
};

export type Language = {
    language_id: string;
    language_name: string;
};

export type Book = {
    object_id: string;
    object_name: string;
    publication_year?: string;
    identifier_id: string;
    language_id: string;
    page_count: number;
};

export type Music = {
    object_id: string;
    object_name: string;
    publication_year?: string;
    identifier_id: string;
    language_id: string;
    duration: string;
};

export type ObjectInCollection = {
    object_id: string;
    collection_id: string;
    object_type: number;
};

export type Country = {
    country_id: string;
    country_name: string;
};

export type Pegi = {
    pegi_id: string;
    pegi_category: string;
};

export type Videogame = {
    object_id: string;
    object_name: string;
    publication_year?: string;
    identifier_id: string;
    language_id: string;
    controller_support: boolean;
    pegi_id: string;
};