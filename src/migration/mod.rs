pub mod cli;
pub mod connection;
pub mod database;
pub mod error;
pub mod manager;
pub mod migrator;
pub mod prelude;
pub mod query;
pub mod seaql_migrations;
pub mod statement;

pub use connection::*;
pub use database::*;
pub use error::*;
pub use manager::*;
pub use migrator::*;
pub use query::*;
pub use statement::*;

use crate::DbErr;

/// Define the name of a migration
pub trait MigrationName {
    /// Get migration name
    fn name(&self) -> &str;
}

/// Define the actions of a migration
#[async_trait::async_trait]
pub trait MigrationTrait: MigrationName + Send + Sync {
    /// Define actions to perform when applying the migration
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr>;

    /// Define actions to perform when rolling back the migration
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr>;
}
