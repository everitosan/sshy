PRAGMA foreign_keys = ON;

-- Table to store groups of servers

DROP TABLE IF EXISTS sshy_group;
CREATE TABLE sshy_group (
  id TEXT NOT NULL PRIMARY KEY,
  parent_id TEXT,
  name TEXT NOT NULL,
  FOREIGN KEY (parent_id) REFERENCES sshy_group(id) ON DELETE CASCADE
);

-- Table to store server

DROP TABLE IF EXISTS sshy_server;
CREATE TABLE sshy_server (
  id TEXT NOT NULL PRIMARY KEY,
  group_id TEXT NOT NULL,
  name TEXT NOT NULL,
  hostname TEXT NOT NULL,
  port INTEGER NOT NULL,
  user TEXT NOT NULL
  FOREIGN KEY (group_id) REFERENCES sshy_group(id) ON DELETE CASCADE
);


-- table to store keys

DROP TABLE IF EXISTS sshy_key_pair;

CREATE TABLE sshy_key_pair (
  id TEXT NOT NULL PRIMARY KEY,
  server_id TEXT NOT NULL,
  FOREIGN KEY (server_id) REFERENCES sshy_server(id) ON DELETE CASCADE
);

DROP TABLE IF EXISTS sshy_key;
CREATE TABLE sshy_key (
  type TEXT CHECK(type IN ('public', 'private')) NOT NULL,
  pair_id TEXT NOT NULL,
  content TEXT NOT NULL,
  FOREIGN KEY (pair_id) REFERENCES sshy_key_pair(id) ON DELETE CASCADE
);
