use tcc_live_lib::*;
use tcc_live_lib::yt_ytils::{cvt_video_id_url};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let pool = mk_connection_pool()
    .map_err(|err| err.to_string())?;

  let mut conn = 
      pool.get().map_err(|err| format!("cannot get connection: {err}"))?;

  let all_record = retrieve_all_info(&mut conn)?;

  // Markdown style print
  all_record.iter().for_each(move |entry| {
    println!("[# {}]({})", entry.title, cvt_video_id_url(entry.live_id.as_str()));
  });

  return Ok(());
}