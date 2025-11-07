

/// Append video ID to standard YouTube video URL.
/// # Examples
/// 
/// ```rust
/// use tcc_live_lib::yt_ytils::cvt_video_id_url;
/// 
/// let video_url = cvt_video_id_url("u_x7T0mT-K4");
/// ```
pub fn cvt_video_id_url(video_id : &str) -> String {
  format!("https://www.youtube.com/watch?v={}", video_id)
}