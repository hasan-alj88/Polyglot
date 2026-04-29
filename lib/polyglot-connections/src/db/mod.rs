use sqlx::{postgres::PgPoolOptions, PgPool, Row};
use std::error::Error;
use serde_json::Value as JsonValue;

/// A wrapper around the sqlx PostgreSQL connection pool.
pub struct PolyglotDb {
    pool: PgPool,
}

impl PolyglotDb {
    /// Connects to the PostgreSQL (NoSQL JSONB) database.
    ///
    /// # Arguments
    /// * `database_url` - The connection string (e.g., "postgres://polyglot:polyglot@localhost/polyglot")
    pub async fn connect(database_url: &str) -> Result<Self, Box<dyn Error>> {
        let pool = PgPoolOptions::new()
            .max_connections(50)
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }

    /// Checks the existence of the NoSQL PostgreSQL database by executing a simple ping query.
    pub async fn check_existence(&self) -> Result<bool, Box<dyn Error>> {
        let result = sqlx::query("SELECT 1")
            .fetch_one(&self.pool)
            .await;

        match result {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Checks if the Mega Metadata Tree (%) topology exists.
    /// In PostgreSQL, this checks if our primary `metadata_tree` table and its JSONB columns exist.
    pub async fn check_metadata_tree_topology(&self) -> Result<bool, Box<dyn Error>> {
        let query = r#"
            SELECT EXISTS (
                SELECT FROM information_schema.tables 
                WHERE table_name = 'metadata_tree'
            );
        "#;

        let exists: bool = sqlx::query(query)
            .fetch_one(&self.pool)
            .await?
            .try_get(0)?;

        Ok(exists)
    }

    /// Initializes the NoSQL DB by creating the fundamental JSONB tables for Polyglot.
    pub async fn initialize_db(&self) -> Result<(), Box<dyn Error>> {
        let create_table_query = r#"
            CREATE TABLE IF NOT EXISTS metadata_tree (
                id VARCHAR(255) PRIMARY KEY,
                path VARCHAR(1024) NOT NULL UNIQUE,
                data JSONB NOT NULL,
                created_at TIMESTAMPTZ DEFAULT NOW(),
                updated_at TIMESTAMPTZ DEFAULT NOW()
            );

            CREATE INDEX IF NOT EXISTS idx_metadata_tree_path ON metadata_tree (path);
            CREATE INDEX IF NOT EXISTS idx_metadata_tree_data ON metadata_tree USING GIN (data);
        "#;

        sqlx::query(create_table_query)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Inserts or updates a node in the Mega Metadata Tree.
    pub async fn upsert_metadata_node(&self, id: &str, path: &str, data: JsonValue) -> Result<(), Box<dyn Error>> {
        let upsert_query = r#"
            INSERT INTO metadata_tree (id, path, data)
            VALUES ($1, $2, $3)
            ON CONFLICT (path) 
            DO UPDATE SET data = EXCLUDED.data, updated_at = NOW();
        "#;

        sqlx::query(upsert_query)
            .bind(id)
            .bind(path)
            .bind(data)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Fetches a JSONB node from the Metadata Tree by its exact path.
    pub async fn get_metadata_node(&self, path: &str) -> Result<Option<JsonValue>, Box<dyn Error>> {
        let select_query = r#"
            SELECT data FROM metadata_tree WHERE path = $1;
        "#;

        let row_opt = sqlx::query(select_query)
            .bind(path)
            .fetch_optional(&self.pool)
            .await?;

        match row_opt {
            Some(row) => {
                let data: JsonValue = row.try_get("data")?;
                Ok(Some(data))
            },
            None => Ok(None)
        }
    }
}
