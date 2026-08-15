-- Accounts (admins and trusted members)
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    is_admin INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Leaderboard = Mode/Category than can be freely created by admins
CREATE TABLE IF NOT EXISTS leaderboards (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    slug TEXT NOT NULL UNIQUE,                  -- ex: "eden-all-chapters"
    title TEXT NOT NULL,                        -- ex: "Eden 4 Goals"
    description TEXT,                           -- ex: "Eden -> BBB/Lamb/Mother/Beast."
    unit TEXT NOT NULL DEFAULT 'Best Streak',   -- displayed unit for score : "Best Streak", "Time", etc.
    stat TEXT NOT NULL DEFAULT 'Status',        -- displayed stat with linked clip : "Status", "Death Count". etc.
    lower_is_better INTEGER NOT NULL DEFAULT 0, -- if 1, lower score is better (usefull for "Time" score)
    created_by INTEGER REFERENCES users(id),
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- An entry in a given leaderboard
CREATE TABLE IF NOT EXISTS entries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    leaderboard_id INTEGER NOT NULL REFERENCES leaderboards(id) ON DELETE CASCADE,
    player_name TEXT NOT NULL COLLATE NOCASE,
    player_link TEXT,                       -- ex: Twitch
    score INTEGER NOT NULL,
    stat_text TEXT NOT NULL DEFAULT '???',  -- ex: Dead/Alive
    stat_link TEXT,                         -- clip link
    note TEXT,                              -- whatever to precise
    added_by INTEGER REFERENCES users(id),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_by Integer REFERENCES users(id),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE (leaderboard_id, player_name)
);

CREATE INDEX IF NOT EXISTS idx_entries_leaderboard ON entries(leaderboard_id);
