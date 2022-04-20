//! Convert SQL statement

use crate::Statement;
use sea_schema::migration;

impl From<migration::Statement> for Statement {
    fn from(stmt: migration::Statement) -> Self {
        Self {
            sql: stmt.sql,
            values: stmt.values,
            db_backend: stmt.db_backend.into(),
        }
    }
}
