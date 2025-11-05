CREATE TABLE tcc_live (
  id        INTEGER   NOT NULL PRIMARY KEY AUTOINCREMENT,
  title     TEXT      NOT NULL DEFAULT "undefined",
  live_url  TEXT      NOT NULL,
  UNIQUE (title, live_url)
)