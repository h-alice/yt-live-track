pub mod models;
pub mod schema;
pub mod yt_util;

// for database

use diesel::{prelude::*};

use diesel::r2d2::{ConnectionManager, Pool};


// for environment
use dotenv::dotenv;
use std::env;

use self::models::{LiveEntry, NewLiveEntry};

use crate::schema::tcc_live;

pub fn mk_connection_pool() 
  -> Result<Pool<ConnectionManager<SqliteConnection>>, Box<dyn std::error::Error>> {
    
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").map_err(|err| format!("env error: {err}"))?;

  let manager = ConnectionManager::<SqliteConnection>::new(database_url);

  Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .map_err(|err| format!("cannot create connection pool {err}").into())

}

pub fn establish_connection() 
  -> Result<SqliteConnection, Box<dyn std::error::Error>> {
    
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("no DATABASE_URL");

  SqliteConnection::establish(&database_url)
    .map_err(|e| format!("unable to connet due to: {e}").into())
}

pub fn insert_live_info(conn: &mut SqliteConnection, title: &str, live_id: &str) 
  -> Result<Option<LiveEntry>, Box<dyn std::error::Error>> {
    
  let new_entry: NewLiveEntry = NewLiveEntry { title, live_id };

  diesel::insert_into(tcc_live::table)
      .values(&new_entry)
      .on_conflict((tcc_live::live_id, tcc_live::title))
      .do_nothing()
      .returning(tcc_live::all_columns)
      .get_result(conn)
      .optional()
      .map_err(|e| format!("inserting failed due to: {e}").into())
}

