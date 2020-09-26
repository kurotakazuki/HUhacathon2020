use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::Serialize;

use crate::schema::{
    tasks,
    tasks::dsl::{participants as task_participants, tasks as all_tasks},
};

#[derive(Debug, Insertable)]
#[table_name = "tasks"]
pub struct NewTask {
    pub title: String,
}

#[derive(Debug, Queryable, Serialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub participants: String,
}

impl Task {
    pub fn all(conn: &PgConnection) -> QueryResult<Vec<Task>> {
        all_tasks.order(tasks::id.desc()).load::<Task>(conn)
    }

    pub fn insert(game: NewTask, conn: &PgConnection) -> QueryResult<usize> {
        diesel::insert_into(tasks::table)
            .values(&game)
            .execute(conn)
    }

    pub fn add_participant_with_id(id: i32, participant: String, conn: &PgConnection) -> QueryResult<usize> {
        let task = all_tasks.find(id).get_result::<Task>(conn)?;

        let mut new_status: String = task.participants.clone();
        new_status.push(' ');
        new_status.push_str(&participant);
        let updated_task = diesel::update(all_tasks.find(id));
        updated_task
            .set(task_participants.eq(new_status))
            .execute(conn)
    }

    pub fn delete_with_id(id: i32, conn: &PgConnection) -> QueryResult<usize> {
        diesel::delete(all_tasks.find(id)).execute(conn)
    }
}
