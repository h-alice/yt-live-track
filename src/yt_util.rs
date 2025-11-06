use reqwest;
use regex::Regex;

/// Append video ID to standard YouTube video URL.
/// # Examples
/// 
/// ```rust
/// let video_url = video_title_from_id("u_x7T0mT-K4");
/// ```
pub fn cvt_video_id_url(video_id : &str) -> String {
  format!("https://www.youtube.com/watch?v={}", video_id)
}

/// Fetch video title from given video id.
/// 
/// # Examples
/// 
/// ```rust
/// let title = video_title_from_id("u_x7T0mT-K4").await?;
/// ```
pub async fn video_title_from_id(video_id : &str) 
  -> Result<String, Box<dyn std::error::Error>> {
    
  // fetch entire page from YT
  let resp: String = reqwest::get(
    format!("https://www.youtube.com/watch?v={}", video_id))
    .await?
    .text()
    .await?;

  let re = Regex::new(r"<title>([^<]*)</title>")?;

  match re.captures(&resp) {
    None => return Err("no title information found".into()),
    Some(cap) => {

      let title = cap.get(1).unwrap().as_str().trim().to_string();

      return Ok(title);

    }
  };
}

/// Fetch *single* live ID from given channel ID.
/// 
/// # Examples
/// 
/// ```rust
/// let live_id = live_id_from_channel_id("UCwiE5sxWHSXD5qEBuAhyzkg").await?
/// ```
pub async fn live_id_from_channel_id(channel_id : &str) 
  -> Result<String, Box<dyn std::error::Error>> {

  let resp: String = reqwest::get(
    format!("https://www.youtube.com/embed/live_stream?channel={}", channel_id))
    .await?
    .text()
    .await?;

  let re = Regex::new(r"https://www\.youtube\.com/watch\?v\\u003d([^\\]*)").unwrap();

  match re.captures(&resp) {
    None => return Err("no live URL found".into()),
    Some(cap) => {
      let live_id = cap.get(1).unwrap().as_str().trim().to_string();
      return Ok(live_id)}
  };
}

/// Fetch *single* live URL from given channel Handle.
/// 
/// Channel handle is something like `SomeChannel` in YouTube URL
/// `https://www.youtube.com/@SomeChannel`
/// 
/// The leading `@` should **not** be included.
/// 
/// # Examples
/// 
/// ```rust
/// let live_id = live_id_from_channel_name("usadapekora").await?
/// ```
pub async fn live_id_from_channel_name(handle : &str) 
  -> Result<String, Box<dyn std::error::Error>> {

  // 1. fetch the full page (there's init data inside)
  use serde_json::Value;

  let resp: String = reqwest::get(
    format!("https://www.youtube.com/@{}", handle))
    .await?
    .text()
    .await?;

  // 2. use re to fetch the raw initial data
  let re = Regex::new(r"var ytInitialData ?= ?(.*?);</script>")?;

  let init_data_raw = re.captures(&resp)
                                    .ok_or("YouTube initial data `ytInitialData` not found")?
                                    .get(1)
                                    .ok_or("cannot parse initial data")?
                                    .as_str()
                                    .trim()
                                    .to_string();

  // 3. load initial data
  let init_data: Value = serde_json::from_str(&init_data_raw)
                                .map_err(|e: serde_json::Error| format!("inserting failed due to: {e}"))?;

  let live_status: Value = init_data["header"]
                              ["pageHeaderRenderer"]
                              ["content"]
                              ["pageHeaderViewModel"]
                              ["image"]
                              ["decoratedAvatarViewModel"].clone();

  match live_status["liveData"]["liveBadgeText"] {
    Value::Null => return Err("no live status found".into()),
    Value::String(_) => { /* no-op, continue */ },
    _ => return Err("no live status found (cannot parse `liveBadgeText`)".into()),
  }      

  let live_id: Value = live_status["rendererContext"]
                          ["commandContext"]
                          ["onTap"]
                          ["innertubeCommand"]
                          ["watchEndpoint"]
                          ["videoId"].clone();

  match live_id {
    Value::String(live_id_parsed) => return Ok(live_id_parsed),
    _ => return Err("got live state, but cannot parse `videoId`".into()),
  }

}