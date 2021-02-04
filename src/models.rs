use diesel::{self, prelude::*};

use crate::schema::dl_info;
use crate::schema::dl_info::dsl::{dl_info as all_dl_info, completed as task_completed};

#[table_name = "dl_info"]
#[derive(Serialize, Queryable, Insertable, Debug, Clone)]
pub struct DlInfo {
    pub id: Option<i32>,
    pub description: Option<String>,
    pub completed: bool,
    pub dl_url: String,
    pub dl_status: String,
    pub dl_progress: Option<f64>,
    pub dl_create_time: String,
    pub dl_end_time: String,
    pub dl_type: String,
    pub file_size: Option<String>,
    pub file_store_path: Option<String>,
}

#[derive(FromForm)]
pub struct Todo {
    pub description: String,
}

impl DlInfo {
    pub fn all(conn: &SqliteConnection) -> Vec<DlInfo> {
        let res = all_dl_info.order(dl_info::id.desc()).load::<DlInfo>(conn);
        res.unwrap()
    }

    pub fn insert(todo: Todo, conn: &SqliteConnection) -> bool {
        let t = DlInfo {
            id: None,
            description: None,
            completed: false,
            dl_url: "".to_string(),
            dl_status: "".to_string(),
            dl_progress: None,
            dl_create_time: "".to_string(),
            dl_end_time: "".to_string(),
            dl_type: "".to_string(),
            file_size: None,
            file_store_path: None,
        };
        diesel::insert_into(dl_info::table).values(&t).execute(conn).is_ok()
    }

    pub fn toggle_with_id(id: i32, conn: &SqliteConnection) -> bool {
        let task = all_dl_info.find(id).get_result::<DlInfo>(conn);
        if task.is_err() {
            return false;
        }

        let new_status = !task.unwrap().completed;
        let updated_task = diesel::update(all_dl_info.find(id));
        updated_task.set(task_completed.eq(new_status)).execute(conn).is_ok()
    }

    pub fn delete_with_id(id: i32, conn: &SqliteConnection) -> bool {
        diesel::delete(all_dl_info.find(id)).execute(conn).is_ok()
    }

    #[cfg(test)]
    pub fn delete_all(conn: &SqliteConnection) -> bool {
        diesel::delete(all_dl_info).execute(conn).is_ok()
    }
}