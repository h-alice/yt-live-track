use tcc_live_lib::*;
use tcc_live_lib::models::*;
use tcc_live_status::yt_utils::*;
use diesel::{prelude::*};
use diesel::r2d2::{ConnectionManager, Pool};

use dotenv::dotenv;
use tokio::select;
use std::env;

/// The full action: First getting live video ID from channel `handle`,
/// then the live title with previously fetched video id, then store 
/// the live information to local SQLite DB.
/// 
/// If bad thing happened, the error will be returned as `String`, convert
/// to `Error` type if needed.
async fn get_live_id_and_insert(pool : Pool<ConnectionManager<SqliteConnection>>, handle : String) 
  -> Result<Option<LiveEntry>, String> {

  // Get the live id
  let live_id: String = 
    live_id_from_channel_name(handle.as_str())
      .await
      .map_err(|_err| format!("<{handle}> {}", _err.to_string()))?; // Stringify error

  let title: String = 
    video_title_from_id(live_id.as_str())
      .await
      .map_err(|err| err.to_string())?; // Stringify error

  // DB action in separated thread
  let res: Option<LiveEntry> = tokio::task::spawn_blocking(move || {

    // 
    let mut conn = 
      pool.get().map_err(|err| format!("cannot get connection: {err}"))?;

    let result = 
      insert_live_info(&mut conn, &title, &live_id);

    match result {
      Ok(inserted) => { Ok(inserted) }
      Err(e) => { Err(e.to_string()) }
    }
    
  }).await
    .unwrap()
    .map_err(|err_str| format!("<{handle}> encounted an error: {err_str}"))?;
  
  return Ok(res);
}

/// The daemon part, it repeatly execute info fetcher, and terminate if
/// received cancel signal.
/// 
/// If bad thing happened, the error will be returned as `String`, convert
/// to `Error` type if needed.
async fn daemon(cancel_tx : tokio::sync::broadcast::Sender<()>) 
  -> Result<(), String> {

  use tokio::time::interval;

  // Initial setup

  dotenv().ok(); // Check .env

  let pool = mk_connection_pool()
    .map_err(|err| err.to_string())?;

  let mut conn = pool.get()
    .map_err(|e| format!("cannot create initial connection: {e}"))?;

  let _ = run_migrations(&mut conn)
    .map_err(|e| format!("cannot run migrations: {e}"))?;

  // init: get channels to track
  let tracking_channels_env = 
    env::var("CHANNELS")
      .map_err(|err| format!("env error: {err}"))?;

  let interval_time_sec_env = 
    env::var("REPEAT_INTERVAL")
      .map(|s| s.parse::<u64>())
      .unwrap()
      .unwrap_or(100);
  
  let tracking_channels = 
    tracking_channels_env.split(";")
      .filter(|x| !x.is_empty())
      .map(|s| s.to_string())
      .collect::<Vec<String>>();

  // init: setup time tick
  // TODO: remove hardcoded time
  let mut tick = interval(tokio::time::Duration::from_secs(interval_time_sec_env));

  // Check if there's any channel name provided.
  if tracking_channels.is_empty() {
    return Err("no channels to track".into());
  }

  // signal watcher
  let mut cancel_rx = cancel_tx.subscribe();
  
  loop { // Actual daemon part

    select! {
      
      _ = tick.tick() => {

        // dispatch handle names into fetcher tasks
        let tasks  = 
          tracking_channels.iter().map(|c| {
            get_live_id_and_insert(pool.clone(), c.to_string())
          });
        
        // create JoinSet from previous result
        let task_set = tokio::task::JoinSet::from_iter(tasks);

        // await task results
        let result = task_set.join_all().await;

        result.iter().for_each(|r| {
          match r {

            Ok(res) => {
              match res {

                Some(new_record) => {
                  println!("[Live Status] inserted new record: {new_record:?}");
                },

                None => {}, // Do nothing
              }
            },

            Err(e) => {
              println!("[Live Status] {:?}", e);
            },
          }
        });

        //println!("result: {:?}", result); // debug print
        
      }

      _ = cancel_rx.recv() => {
        println!("[Daemon] Shutdown signal received. Cleaning up...");
        // TODO: maybe some cleanup here.
        println!("[Daemon] Task terminated gracefully.");

        return Ok(()); // bye bye
      }
    }
  }
}

async fn cancel_sig_watcher(tx : tokio::sync::broadcast::Sender<()>)
  -> Result<(), String> {

  use tokio::signal;
  use tokio::signal::unix::{signal, SignalKind};


  let mut signal_stream = 
    signal(SignalKind::terminate()).map_err(|e| e.to_string())?;

  tokio::select! {

    _ = signal::ctrl_c() => {
      println!("[Watcher] Received Ctrl+C signal.");
    }

    _ = signal_stream.recv() => {
      println!("[Watcher] Received SIGTERM signal.");
    }
  }

  // Send the shutdown signal.
  tx.send(()).map(|_| ()).map_err(|e| e.to_string())

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

  use tokio::sync::broadcast;

  let (cancel_tx , mut _cancel_rx) = 
    broadcast::channel::<()>(5);

  // install the signal handler.
  let hdle_watcher = 
    tokio::spawn(cancel_sig_watcher(cancel_tx.clone()));

  // start the daemon.
  let hdle_daemon = 
    tokio::spawn(daemon(cancel_tx.clone()));

  let _daemon_result = 
    hdle_daemon.await.unwrap().map_err(|e| format!("[Init] daemon terminated with error: {e}"))?; 

  // if we reach this statement, that means daemon terminated,
  // and there's no more needs for the signal watcher.

  // If the daemon terminated due to an error, then the previous `?`,
  // would have passed it to outer runtime, so we won't reach here.

  // If the daemon ended normally due to os signal, then the watcher
  // thread is ended now, so there's no concern to call `await`
  // instead of `abort`.

  let _ = hdle_watcher.await?;

  return Ok(()); // bye bye
}
