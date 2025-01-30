//! Database module to handle player registration,player stats and login using SQLite

use sqlx::sqlite::SqlitePool;
use uuid::Uuid;

#[derive(Debug)]
pub struct PlayerStats {
    pub games_played: i32,
    pub games_won: i32,
}
#[derive(Debug)]
pub struct Player {
    pub id: String,
    pub name: String,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(db_name: &str) -> Result<Self> {
        let conn = Connection::open(db_name)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS players (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL
            )",
            [],
        )?;
        Ok(Database { conn })
    }

    pub fn register_player(&self, name: &str) -> Result<String> {
        let id = Uuid::new_v4().to_string();
        self.conn.execute(
            "INSERT INTO players (id, name) VALUES (?1, ?2)",
            &[&id, name],
        )?;
        Ok(id)
    }

    pub fn login_player(&self, name: &str) -> Result<Option<String>> {
        let mut stmt = self.conn.prepare("SELECT id FROM players WHERE name = ?1")?;
        let mut rows = stmt.query([name])?;

        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }

    pub fn player_stats(&self, player_id: &str) -> Result<PlayerStats> {
        let mut stmt = self.conn.prepare(
            "SELECT COUNT(*) AS games_played, SUM(won) AS games_won
            FROM games
            WHERE player_id = ?1",
        )?;
        let stats = stmt.query_row([player_id], |row| {
            Ok(PlayerStats {
                games_played: row.get(0)?,
                games_won: row.get(1)?,
            })
        })?;
        Ok(stats)
    }

    pub fn list_players(&self) -> Result<Vec<Player>> {
        let mut stmt = self.conn.prepare("SELECT id, name FROM players")?;
        let players = stmt.query_map([], |row| {
            Ok(Player {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
        Ok(players)
    }
}
