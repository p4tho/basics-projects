CREATE TABLE IF NOT EXISTS agents (
    id                      INTEGER PRIMARY KEY,
    name                    TEXT NOT NULL UNIQUE CHECK (length(trim(name)) > 0),
    system_name             TEXT,
    hostname                TEXT,
    os                      TEXT,
    os_version              TEXT,
    kernel_version          TEXT,
    cpu                     TEXT
);

-- Will eventully tie in with results and commands
CREATE TABLE IF NOT EXISTS beacons (
    id                      INTEGER PRIMARY KEY,
    agent_id                INTEGER NOT NULL CHECK (agent_id > 0),
    timestamp               DATETIME DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (agent_id) REFERENCES agents(id)
);

CREATE TABLE IF NOT EXISTS commands (
    id                      INTEGER PRIMARY KEY,
    agent_id                INTEGER NOT NULL CHECK (agent_id > 0),
    command_type            INTEGER NOT NULL CHECK (command_type >= 0),
    cmd                     TEXT,
    status                  TEXT NOT NULL DEFAULT 'pending',
    result                  TEXT DEFAULT '',

    FOREIGN KEY (agent_id) REFERENCES agents(id)
);