# yt-live-track
A small tool that check a channel's live status, and store the live video id to local database.

因為臺北市議會總是不公開YouTube上的質詢連結😡，為了記錄所以快速刻了這個工具。

You can list target channel in `CHANNELS` section in `.env`, it has the ability of tracking multiple channels. (list all name in one line, separated bt `;`)

Currently, the tool only accepts channel handle name (e.g. `臺北市議會-110`, without the `@` in channel URL), we might add the compatability for channel id in the future.