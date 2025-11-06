pub mod models;
pub mod schema;
pub mod yt_util;

// for database

use diesel::prelude::*;

// for environment
use dotenv::dotenv;
use std::env;

use self::models::{LiveEntry, NewLiveEntry};

use crate::schema::tcc_live;

pub fn establish_connection() 
  -> Result<SqliteConnection, Box<dyn std::error::Error>> {
    
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("no DATABASE_URL");

  SqliteConnection::establish(&database_url)
    .map_err(|e| format!("unable to connet due to: {e}").into())
}

pub fn insert_live_info(conn: &mut SqliteConnection, title: &str, url: &str) 
  -> Result<LiveEntry, Box<dyn std::error::Error>> {
    
  let new_entry: NewLiveEntry = NewLiveEntry { title, live_id: url };

  diesel::insert_into(tcc_live::table)
      .values(&new_entry)
      .returning(LiveEntry::as_returning())
      .get_result(conn)
      .map_err(|e| format!("inserting failed due to: {e}").into())
}

