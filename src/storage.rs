use chrono::{Local};
use rusqlite::{Connection, params};

pub fn db() -> Connection {
    let conn = Connection::open("routine.db").unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sessions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            app TEXT,
            activity TEXT,
            start_ts INTEGER,
            end_ts INTEGER
        )", [],
    ).unwrap();
    conn
}

pub fn start_session(app: &str, activity: &str, start_ts: i64) {
    let conn = db();
    conn.execute(
        "INSERT INTO sessions (app, activity, start_ts, end_ts)
        VALUES (?1, ?2, ?3, NULL)",
        params![app, activity, start_ts],
    ).unwrap();
}

pub fn end_last_session(end_ts: i64) {
    let conn = db();
    conn.execute(
        "UPDATE sessions SET end_ts=?1 WHERE end_ts IS NULL",
        params![end_ts],
    ).unwrap();
}

pub fn fetch_sessions_between(from_ts: i64, to_ts: i64) -> Vec<(String, String, i64, i64)> {
    let conn = db();
    let mut stmt = conn
        .prepare("SELECT app, activity, start_ts, end_ts FROM sessions 
                  WHERE start_ts>=?1 AND start_ts<?2 AND end_ts IS NOT NULL")
        .unwrap();

    let rows = stmt
        .query_map(params![from_ts, to_ts], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get::<_, i64>(2)?,
                row.get::<_, i64>(3)?,
            ))
        })
        .unwrap();

    rows.map(|r| r.unwrap()).collect()
}
