use std::ops::Index;

use crate::db::db_connection::{get_sqlite, sql_operations};
use crate::db::db_models::{Downloads, Schedules, Settings, CRUD};
use crate::{limit_order, sql_c};

impl CRUD for Downloads {
    fn create<T>(&self) -> i32 {
        sql_c!(sqlite);
        sqlite.execute(&format!(
            "
            INSERT INTO downloads (status, output, url, file_size, types, version, category, divise)
            VALUES ({}, '{}', '{}', {}, '{}', '{}', '{}', '{}');
            ",
            self.status,
            self.output,
            self.url,
            self.file_size,
            self.types,
            self.version,
            self.category,
            self.divise
        ));

        let mut get_id = sqlite
            .conn
            .prepare(&format!(
                "
                SELECT id
                FROM downloads
                WHERE output = '{}' AND status = 0
                LIMIT 1;
                ",
                self.output
            ))
            .unwrap();
        let all_ids: Result<Vec<i32>, rusqlite::Error> = get_id
            .query_map([], |row| Ok(row.get(0).unwrap()))
            .unwrap()
            .collect();
        *all_ids.unwrap().get(0).unwrap()
    }

    fn readall<T>(&self,limit:i32,sort_by:&str) -> Vec<Downloads> {
        sql_c!(sqlite);
        limit_order!(limit,sort_by,limit_query,order_by_query);
        
        let mut get_id = sqlite
            .conn
            .prepare(&format!(
                "
                    SELECT *
                    FROM downloads
                    {}
                    {}
                    ;
        ",order_by_query,limit_query
            ))
            .expect("an error occured on running sql commands maybe wrong column");
        let all_ids: Result<Vec<Downloads>, rusqlite::Error> = get_id
            .query_map([], |row| {
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
            .unwrap()
            .collect();
        all_ids.unwrap()
    }

    fn update<T>(&self) -> Self {
        sql_c!(sqlite);
        let _ = sqlite
            .conn
            .execute(
                &format!(
                    "
        UPDATE downloads
        SET status = {}, output = '{}', url = '{}', file_size = {}
        WHERE id = {};
    ",
                    self.status,
                    self.output,
                    self.url,
                    self.file_size,
                    self.id.unwrap()
                ),
                [],
            )
            .unwrap();
        self.read_by_id::<Downloads>(self.id.unwrap())
    }

    fn delete<T>(&self, id: T) -> Self {
        todo!()
    }

    fn read_by_id<T>(&self, id: i32) -> Self {
        sql_c!(sqlite);
        let mut get_id = sqlite
            .conn
            .prepare(&format!(
                "
                SELECT id, status, output, url, file_size, types, version, category, divise
                FROM downloads
                WHERE id = {} LIMIT 1;
                ",
                id
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
            });
        match all_ids {
            Ok(download) => download,
            Err(_) => Downloads::blank(),
        }
    }
}

impl CRUD for Schedules {
    fn create<T>(&self) -> i32 {
        sql_c!(sqlite);
        sqlite.execute(&format!(
            "
        INSERT INTO schedules (id, download_id, time,etime, day)
VALUES (NULL, {}, '{}','{}', '{}');
        ",
            self.download_id, self.time,self.etime, self.day
        ));
        let mut get_id = sqlite
            .conn
            .prepare(&format!(
                "
        SELECT id
FROM schedules
WHERE time = '{}' 
ORDER BY time DESC
LIMIT 1;
        ",
                self.time
            ))
            .unwrap();
        let all_ids: Result<Vec<i32>, rusqlite::Error> = get_id
            .query_map([], |row| Ok(row.get(0).unwrap()))
            .unwrap()
            .collect();
        *all_ids.unwrap().get(0).unwrap_or_else(||&-1)
    }

    fn readall<T>(&self,limit:i32,sort_by:&str) -> Vec<Schedules> {
        sql_c!(sqlite);
        limit_order!(limit,sort_by,limit_query,order_by_query);
        let mut get_id = sqlite
            .conn
            .prepare(&format!(
                "
                    SELECT *
                    FROM schedules
                    {}
                    {}
                    ;
        ",order_by_query,limit_query
            ))
            .expect("an error occured on running sql commands maybe wrong column");
        let all_ids: Result<Vec<Schedules>, rusqlite::Error> = get_id
            .query_map([], |row| {
                Ok(Schedules::new(
                    row.get(0).unwrap(),
                    row.get(1).unwrap(),
                    row.get(2).unwrap(),
                    row.get(3).unwrap(),
                    row.get(4).unwrap(),
                ))
            })
            .unwrap()
            .collect();
        all_ids.unwrap()
    }

    fn update<T>(&self) -> Schedules {
        sql_c!(sqlite);
        let _ = sqlite
            .conn
            .execute(
                &format!(
                    "
        UPDATE schedules
        SET download_id = {}, time = '{}', day = '{}'
        WHERE id = {};
    ",
                    self.download_id,
                    self.time,
                    self.day,
                    self.id
                ),
                [],
            )
            .unwrap();
        self.read_by_id::<Schedules>(self.id)
    }

    fn delete<T>(&self, id: T) -> Schedules {
        todo!()
    }

    fn read_by_id<T>(&self, id: i32) -> Schedules {
        sql_c!(sqlite);
        let mut get_id = sqlite
            .conn
            .prepare(&format!(
                "
        SELECT *
FROM schedules
WHERE id = {} LIMIT 1;
        ",
                id
            ))
            .unwrap();
        let all_ids = get_id
            .query_row([], |row| {
                Ok(Schedules::new(
                    row.get(0).unwrap(),
                    row.get(1).unwrap(),
                    row.get(2).unwrap(),
                    row.get(3).unwrap(),
                    row.get(4).unwrap(),
                ))
            })
            .unwrap();
        all_ids
    }
}

impl CRUD for Settings {
    fn create<T>(&self) -> i32 {
        sql_c!(sqlite);
        sqlite.execute(&format!(
            "
        INSERT INTO settings (id, name, value)
VALUES (NULL, '{}', '{}');
        ",
            self.name, self.value
        ));
        let mut get_id = sqlite
            .conn
            .prepare(&format!(
                "
        SELECT id
FROM settings
WHERE name = '{}' 
AND
value = '{}' 
ORDER BY name DESC
LIMIT 1;
        ",
        self.name, self.value
            ))
            .unwrap();
        let all_ids: Result<Vec<i32>, rusqlite::Error> = get_id
            .query_map([], |row| Ok(row.get(0).unwrap()))
            .unwrap()
            .collect();
        *all_ids.unwrap().get(0).unwrap()
    }

    fn readall<T>(&self,limit:i32,sort_by:&str) -> Vec<Settings> {
        sql_c!(sqlite);
        limit_order!(limit,sort_by,limit_query,order_by_query);
        let mut get_id = sqlite
            .conn
            .prepare(&format!(
                "
                    SELECT *
                    FROM settings
                    {}
                    {}
                    ;
        ",order_by_query,limit_query
            ))
            .expect("an error occured on running sql commands maybe wrong column");
        let all_ids: Result<Vec<Settings>, rusqlite::Error> = get_id
            .query_map([], |row| {
                Ok(Settings::new(
                    row.get(0).unwrap(),
                    row.get(1).unwrap(),
                    row.get(2).unwrap()
                ))
            })
            .unwrap()
            .collect();
        all_ids.unwrap()
    }

    fn update<T>(&self) -> Settings {
        sql_c!(sqlite);
        let _ = sqlite
            .conn
            .execute(
                &format!(
                    "
        UPDATE settings
        SET name = '{}', value = '{}'
        WHERE id = {};
    ",
                    self.name,
                    self.value,
                    self.id
                ),
                [],
            )
            .unwrap();
        self.read_by_id::<Settings>(self.id)
    }

    fn delete<T>(&self, id: T) -> Settings {
        todo!()
    }

    fn read_by_id<T>(&self, id: i32) -> Settings {
        sql_c!(sqlite);
        let mut get_id = sqlite
            .conn
            .prepare(&format!(
                "
        SELECT *
FROM settings
WHERE id = {} LIMIT 1;
        ",
                id
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