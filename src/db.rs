use std::ops::Deref;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};

use crate::model::{NewTask, Task};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

fn get_conn(pool: &PgPool) -> Result<PgPooledConnection, &'static str> {
    pool.get().map_err(|_| "Can't get connection")
}

pub fn get_all_tasks(pool: &PgPool) -> Result<Vec<Task>, &'static str> {
    Task::all(get_conn(pool)?.deref()).map_err(|_| "Error retrieving tasks")
}

pub fn create_task(todo: String, pool: &PgPool) -> Result<(), &'static str> {
    let new_task = NewTask { title: todo };
    Task::insert(new_task, get_conn(pool)?.deref())
        .map(|_| ())
        .map_err(|_| "Error inserting task")
}

pub fn add_participant_task(id: i32, participant: String, pool: &PgPool) -> Result<(), &'static str> {
    Task::add_participant_with_id(id, participant, get_conn(pool)?.deref())
        .map(|_| ())
        .map_err(|_| "Error add participant task completion")
}

pub fn delete_task(id: i32, pool: &PgPool) -> Result<(), &'static str> {
    Task::delete_with_id(id, get_conn(pool)?.deref())
        .map(|_| ())
        .map_err(|_| "Error deleting task")
}
