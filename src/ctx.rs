use chrono::{DateTime, Utc};
use nanoid::nanoid;
use sqlx::{mysql::MySql, pool::Pool};

#[derive(Debug, Clone)]
pub(crate) struct Context {
    pub(crate) execution_start_time: String,
    pub(crate) start: DateTime<Utc>,
    pub(crate) pool: Pool<MySql>,
}

impl Context {
    pub(crate) fn new(pool: Pool<MySql>) -> Self {
        Self {
            execution_start_time: nanoid!(),
            start: Utc::now(),
            pool,
        }
    }
}
