//! Manage migration connection

use crate::{ConnectionTrait, DatabaseConnection};
use sea_schema::migration::{self, MigrationErr, QueryResultTrait};

#[async_trait::async_trait]
impl migration::ConnectionTrait for DatabaseConnection {
    fn get_database_backend(&self) -> migration::DbBackend {
        ConnectionTrait::get_database_backend(self).into()
    }

    async fn execute(&self, stmt: migration::Statement) -> Result<(), MigrationErr> {
        ConnectionTrait::execute(self, stmt.into())
            .await
            .map(|_| ())
            .map_err(|e| MigrationErr(e.to_string()))
    }

    async fn query_one(
        &self,
        stmt: migration::Statement,
    ) -> Result<Option<Box<dyn QueryResultTrait>>, MigrationErr> {
        ConnectionTrait::query_one(self, stmt.into())
            .await
            .map(|res| res.map(|res| Box::new(res) as Box<dyn QueryResultTrait>))
            .map_err(Into::into)
    }

    async fn query_all(
        &self,
        stmt: migration::Statement,
    ) -> Result<Vec<Box<dyn QueryResultTrait>>, MigrationErr> {
        ConnectionTrait::query_all(self, stmt.into())
            .await
            .map(|rows| {
                rows.into_iter()
                    .map(|row| Box::new(row) as Box<dyn QueryResultTrait>)
                    .collect()
            })
            .map_err(Into::into)
    }
}
