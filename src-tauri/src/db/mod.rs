use std::env;

use diesel::prelude::*;
use diesel::{Connection, PgConnection};
use dotenvy::dotenv;
use log::{error, warn};

use crate::db::models::Book;

mod models;
mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv()
        .inspect_err(|e| {
            warn!("Dotenv not available: {}", e);
            warn!("Continuing");
        })
        .ok();

    let database_url = env::var("DATABASE_URL")
        .inspect_err(|_| error!("DATABASE_URL should be set"))
        .unwrap();

    PgConnection::establish(&database_url)
        .inspect_err(|_| error!("Error Connecting to {}", database_url))
        .unwrap()
}

pub fn get_book(id: String) {
    use schema::libro::dsl::*;

    let connection = &mut establish_connection();
    let result: Vec<Book> = libro
        .filter(id_objeto.eq(id))
        .limit(1)
        .select(Book::as_select())
        .load(connection)
        .expect("Error loading books");

    println!("{:?}", result);
}
