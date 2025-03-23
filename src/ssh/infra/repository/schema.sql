-- Enable Foreign keys
PRAGMA foreign_keys = ON;

-- Disable WAL mode
PRAGMA journal_mode = DELETE;

-- Credential Storage for a connection
DROP TABLE IF EXISTS sshy_credentials;
CREATE TABLE sshy_credentials
(
  -- UUID of keypair
  id          TEXT NOT NULL UNIQUE,
  -- Name to identfy this credentials
  name        TEXT NOT NULL,
  -- User fot the connection
  user        TEXT NOT NULL,
  -- Public key for ssh connection
  public_key  TEXT NOT NULL,
  -- Private key for ssh connection
  private_key TEXT NOT NULL,
  -- Flag to set registration in remote server is ok
  registered  BOOL NOT NULL DEFAULT False,
  -- Creation datetime
  created     TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (id)
);

-- Group store
DROP TABLE IF EXISTS sshy_group;
CREATE TABLE sshy_group
(
  -- UUID
  id        TEXT NOT NULL UNIQUE,
  -- Enable sub-groups
  parent_id TEXT,
  -- Name of the group
  name      TEXT NOT NULL,
  -- Flag for logic delete
  deleted   BOOL NOT NULL DEFAULT False,
  -- Creation datetime
  created   TEXT NOT NULL UNIQUE DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (id),
  FOREIGN KEY (parent_id) REFERENCES sshy_group (id) ON DELETE CASCADE
);

-- Server store
DROP TABLE IF EXISTS sshy_server;
CREATE TABLE sshy_server
(
  -- UUID
  id       TEXT    NOT NULL,
  -- Group UUID
  group_id TEXT    NOT NULL,
  -- Name of server
  name     TEXT    NOT NULL,
  -- Ip or domain of the server
  hostname TEXT    NOT NULL,
  -- Port for ssh connection
  port     INTEGER NOT NULL,
  -- Flag for logic delete
  deleted  BOOL    NOT NULL DEFAULT False,
  -- Creation datetime
  created  TEXT    NOT NULL UNIQUE DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (id),
  FOREIGN KEY (group_id) REFERENCES sshy_group (id) ON DELETE CASCADE
);

-- Relation of credentials with server
DROP TABLE IF EXISTS sshy_server_credentials;
CREATE TABLE sshy_server_credentials
(
  -- UUID of server
  server_id      TEXT NOT NULL,
  -- UUID of credentials
  credentials_id TEXT NOT NULL,
  FOREIGN KEY (server_id) REFERENCES sshy_server (id) ON DELETE CASCADE,
  FOREIGN KEY (credentials_id) REFERENCES sshy_credentials (id) ON DELETE CASCADE
);