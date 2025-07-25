use std::sync::Mutex;

use diesel::PgConnection;

pub struct AppState {
    pub posgresql: Mutex<PgConnection>,
}

impl AppState {
    pub fn new() -> Result<AppState, diesel::ConnectionError> {
        Ok(AppState {
            posgresql: Mutex::new(crate::db::establish_connection()?),
        })
    }
}
