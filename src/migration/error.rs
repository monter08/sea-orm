//! Convert migration error

use crate::DbErr;
use sea_schema::migration;

impl From<DbErr> for migration::MigrationErr {
    fn from(val: DbErr) -> Self {
        migration::MigrationErr(val.to_string())
    }
}

impl From<migration::MigrationErr> for DbErr {
    fn from(migration_err: migration::MigrationErr) -> Self {
        Self::Migration(migration_err.to_string())
    }
}
