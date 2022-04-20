//! Get query result from db

use crate::QueryResult;
use sea_schema::migration::{self, MigrationErr};

impl migration::QueryResultTrait for QueryResult {
    fn try_get_string(&self, col: &str) -> Result<String, MigrationErr> {
        self.try_get::<String>("", col).map_err(Into::into)
    }

    fn try_get_i64(&self, col: &str) -> Result<i64, MigrationErr> {
        self.try_get::<i64>("", col).map_err(Into::into)
    }
}
