use crate::db::db_connection::get_sqlite;
use crate::sql_c;
use serde::Serialize;


#[derive(Serialize, serde::Deserialize, Debug)]
pub struct Downloads {
    pub id: Option<i32>,
    pub status: i32,
    pub output: String,
    pub url: String,
    pub file_size: u64,
    pub types: String,    
    pub version: String, 
    pub category: String,
    pub divise: String,   
}
impl Downloads {
    pub fn new(id: Option<i32>, status: i32, output: String, url: String, file_size: u64, types: String, version: String, category: String, divise: String) -> Self {
        Self {
            id,
            status,
            output,
            url,
            file_size,
            types,
            version,
            category,
            divise,
        }
    }

    pub fn blank() -> Self {
        Downloads::new(Some(0), 0, "".to_string(), "".to_string(), 0, "".to_string(), "".to_string(), "".to_string(), "".to_string())
    }
    pub fn get_download_with_status_and_name(&self) -> Self {
        sql_c!(sqlite);
        let mut get_id = sqlite
            .conn
            .prepare(&format!(
                "
    SELECT *
FROM downloads
WHERE output='{}' LIMIT 1;
    ",
                self.output
            ))
            .unwrap();
        let all_ids = get_id
            .query_row([], |row| {
                Ok(Downloads::new(
                    row.get(0).unwrap(),
                    row.get(1).unwrap(),
                    row.get(2).unwrap(),
                    row.get(3).unwrap(),
                    row.get(4).unwrap(),
                    row.get(5).unwrap(),
                    row.get(6).unwrap(),
                    row.get(7).unwrap(),
                    row.get(8).unwrap(),
                ))
            })
            .unwrap();
        all_ids
    }
    pub fn set_force_exit_to_active_downloads() {
        sql_c!(sqlite);
        let mut get_id = sqlite
            .conn
            .execute(&format!(
                "
                    Update downloads
                SET status = 1
                WHERE status = 2;
                    "

            ),[])
            .unwrap();
    }
    pub fn get_active_downloads() -> Vec<i32> {
        sql_c!(sqlite);
        let mut get_id = sqlite
            .conn
            .prepare(&format!(
                "
    SELECT *
FROM downloads
WHERE status = 1;
    "
            ))
            .unwrap();
        let all_ids: Result<Vec<i32>, rusqlite::Error> = get_id
            .query_map([], |row| Ok(row.get(0).unwrap()))
            .unwrap()
            .collect();
        all_ids.unwrap()
    }
}
#[derive(Serialize, serde::Deserialize, Debug)]
pub struct Schedules {
    pub id: i32,
    pub download_id: i32,
    pub time: String,
    pub etime: String,
    pub day: String,
}
impl Schedules {
    pub fn new(id: i32, download_id: i32, time: String,etime: String, day: String) -> Self {
        Self {
            id,
            download_id,
            time,
            etime,
            day,
        }
    }
    pub fn blank() -> Self {
        Schedules::new(0, 0, 0.to_string(), 0.to_string(),0.to_string())
    }
    pub fn today_schedules(&self) -> Vec<Self> {
        sql_c!(sqlite);
        let mut get_id = sqlite
            .conn
            .prepare(&format!(
                "
SELECT *
FROM schedules
WHERE day = '{}' 
ORDER BY time DESC
",
                self.day
            ))
            .unwrap();
        let all_ids: Result<Vec<Schedules>, rusqlite::Error> = get_id
            .query_map([], |row| Ok(
                Schedules::new(
                    row.get(0).unwrap(),
                    row.get(1).unwrap(),
                    row.get(2).unwrap(),
                    row.get(3).unwrap(),
                    row.get(4).unwrap(),
                )
            ))
            .unwrap()
            .collect();
        all_ids.expect("get wrong with getting data")
    }
}
#[derive(Serialize, serde::Deserialize, Debug)]
pub struct Settings {
    pub id: i32,
    pub name: String,
    pub value: String,
}
impl Settings {
    pub fn new(id: i32, name: String, value: String) -> Self {
        Self { id, name, value }
    }
    pub fn blank() -> Self {
        Settings::new(0, "".to_string(), "".to_string())
    }
    pub fn find_by_key(key: &str) -> Settings {
        sql_c!(sqlite);
        let mut get_id = sqlite
            .conn
            .prepare(&format!(
                "
                SELECT *
        FROM settings
        WHERE name = '{}' LIMIT 1;
                ",
                key
            ))
            .unwrap();
        let all_ids = get_id
            .query_row([], |row| {
                Ok(Settings::new(
                    row.get(0).unwrap(),
                    row.get(1).unwrap(),
                    row.get(2).unwrap(),
                ))
            });
            match all_ids {
                Ok(_) =>all_ids.unwrap(),
                Err(_) => {
                    println!("cant unwrap");
                    Settings::blank()
                },
            }
        
    }
}
pub trait CRUD {
    fn create<T>(&self) -> i32;
    fn readall<T>(&self, limit: i32, sort_by: &str) -> Vec<Self>
    where
        Self: Sized;
    fn read_by_id<T>(&self, id: i32) -> Self;
    fn update<T>(&self) -> Self;
    fn delete<T>(&self, id: T) -> Self;
}
