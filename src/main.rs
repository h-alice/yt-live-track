use reqwest;
use regex::Regex;
//use std::fmt;
use std::error::Error;

static YT_URL_PAT : &str = r"https://www\.youtube\.com/watch\?v\\u003d[^\\]*";
static YT_TITLE_PAT : &str = r"<title>([^<]*)</title>";

/// Fetch video title from given video id.
/// 
/// # Examples
/// 
/// ```rust
/// let title = video_title_from_id("u_x7T0mT-K4").await?;
/// ```
async fn video_title_from_id(video_id : &str) 
    -> Result<String, Box<dyn Error>> {

    // fetch entire page from YT
    let resp: String = reqwest::get(
        format!("https://www.youtube.com/watch?v={}", video_id))
        .await?
        .text()
        .await?;

    let re = Regex::new(YT_TITLE_PAT)?;

    match re.captures(&resp) {
        None => return Err("no title information found".into()),
        Some(cap) => {
            let title = cap.get(1).unwrap().as_str().to_string();
            return Ok(title);
        }
    };
}

/// Fetch `single` live URL from given channel ID.
/// 
/// # Examples
/// 
/// ```rust
/// let live_url = live_url_from_channel("UCwiE5sxWHSXD5qEBuAhyzkg").await?
/// ```
async fn live_url_from_channel(channel_id : &str) 
    -> Result<String, Box<dyn Error>> {

    let resp: String = reqwest::get(
        format!("https://www.youtube.com/embed/live_stream?channel={}", channel_id))
        .await?
        .text()
        .await?;

    let re = Regex::new(YT_URL_PAT).unwrap();

    match re.find(&resp) {
        None => return Err("no live URL found".into()),
        Some(m) => {
            let converted_live_url = m.as_str().to_string().replace(r"\u003d", "=");
            return Ok(converted_live_url)}
    };
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    
    let result = live_url_from_channel("UCwiE5sxWHSXD5qEBuAhyzkg").await;
    
    match result{
        Ok(x) => {
            println!("{}", x);
        }
        Err(_e) => {
            println!("We cannot find live URL from given channel.")
        }
    }

    let result2 = video_title_from_id("u_x7T0mT-K4").await?;
    println!("{}", result2);

   //println!("Hello, world!");
   //println!("{:?}", resp);

    return Ok(());
}
