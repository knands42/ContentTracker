CREATE TABLE IF NOT EXISTS animes (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    episodes_watched INTEGER NOT NULL,
    total_episodes INTEGER,
    status TEXT CHECK(status IN ('Watching', 'Completed', 'Paused', 'Dropped', 'PlanToWatch')) NOT NULL,
    added_at TEXT NOT NULL
)
