use std::env;

use diesel::{Connection, PgConnection};
use dotenvy::dotenv;
use log::{error, warn};

pub mod models;
pub mod schema;

pub fn establish_connection() -> Result<PgConnection, diesel::ConnectionError> {
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
}
