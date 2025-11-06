use tcc_live_capture::*;
use tcc_live_capture::models::*;
use tcc_live_capture::yt_util::*;
use diesel::{prelude::*};
use diesel::r2d2::{ConnectionManager, Pool};

use dotenv::dotenv;
use std::env;


//use std::fmt;


async fn get_live_id_and_insert(pool : &Pool<ConnectionManager<SqliteConnection>>, handle : &str) 
  -> Result<Option<LiveEntry>, Box<dyn std::error::Error>> {

  let pool = pool.clone();
  let live_id = live_id_from_channel_name(handle).await?;
  let title = video_title_from_id(live_id.as_str()).await?;

  let res = tokio::task::spawn_blocking(move || {
    let mut conn = pool.get().map_err(|err| format!("cannot get connection: {err}"))?;
    let result = insert_live_info(&mut conn, &title, &live_id);
    match result {
      Ok(inserted) => { Ok(inserted) }
      Err(e) => { Err(e.to_string()) }
    }
  }).await?.map_err(|err_str| err_str)?;
  
  return Ok(res);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

  dotenv().ok();
  let tracking_channels_env = env::var("CHANNELS").map_err(|err| format!("env error: {err}"))?;

  let tracking_channels = tracking_channels_env.split(";").filter(|x| !x.is_empty()).collect::<Vec<&str>>();

  if tracking_channels.is_empty() {
    return Err("no channels to track".into());
  }

  let pool = mk_connection_pool()?;

  let res = get_live_id_and_insert(&pool, tracking_channels[0]).await?;

  match res {
    Some(row) => println!("successfull insert: \n{:?}", row),
    None => println!("Live database remain unchanged, record may have already existed"),
  }

  return Ok(());
}
