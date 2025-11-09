# yt-live-track
A small tool that checks a channel's live status and stores the live video id to a local database.

因為臺北市議會總是不公開YouTube上的質詢連結😡，為了記錄所以快速刻了這個工具。

You can list target channels in the `CHANNELS` section in `.env`, it has the ability to track multiple channels. (list all names in one line, separated by `;`)

Currently, the tool only accepts channel handle names (e.g. `臺北市議會-110`, without the `@` in the channel URL), we might add compatibility for channel IDs in the future.