//! Convert database backend

use crate::DbBackend;
use sea_schema::migration;

impl From<DbBackend> for migration::DbBackend {
    fn from(val: DbBackend) -> Self {
        match val {
            DbBackend::MySql => migration::DbBackend::MySql,
            DbBackend::Postgres => migration::DbBackend::Postgres,
            DbBackend::Sqlite => migration::DbBackend::Sqlite,
        }
    }
}

impl From<migration::DbBackend> for DbBackend {
    fn from(migration_db_backend: migration::DbBackend) -> Self {
        match migration_db_backend {
            migration::DbBackend::MySql => DbBackend::MySql,
            migration::DbBackend::Postgres => DbBackend::Postgres,
            migration::DbBackend::Sqlite => DbBackend::Sqlite,
        }
    }
}
