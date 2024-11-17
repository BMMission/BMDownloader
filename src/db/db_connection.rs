use rusqlite::{Connection, Result};

use crate::global::Downloader_Settings;
pub struct sql_operations{
    pub conn:Connection,
    path:Option<String>
}
pub enum ConnectionType {
    MEMORY,
    FILE
}
impl sql_operations {
   
    pub fn new(conn_type:ConnectionType,sql_path:Option<String>)->Self{
        match conn_type {
            ConnectionType::MEMORY => {
                Self{
                    conn:Connection::open_in_memory().expect("cant make in memory dbs"),
                    path:None
                }
            }
            ConnectionType::FILE => {
                Self{ 
                    conn: Connection::open(sql_path.clone().expect("something wrong with sql_path.double check sql_path argument")).expect("cant open sqlite file.please check file is not corrupted or in read-only mode"), 
                    path: sql_path
                }
            }
        }
    }
    pub fn execute(&self,query:&str)->usize{
        match self.conn.execute(query, ())  {
            Ok(size) => size ,
            Err(e) => {
                println!("get error to execute sql");
                println!("{}",e);
                0
            },
        }
    }
    pub fn init_db(&self) {
        let sql_db = "
            CREATE TABLE downloads (
                id INTEGER NOT NULL PRIMARY KEY,
                status INTEGER NOT NULL DEFAULT 0,
                output TEXT NOT NULL,
                url TEXT NOT NULL,
                file_size NUMERIC NOT NULL,
                types TEXT NOT NULL,      
                version TEXT NOT NULL,    
                category TEXT NOT NULL,   
                divise TEXT NOT NULL      
            );
            ";
self.execute(sql_db);
        let sql_db="CREATE TABLE schedules (
                                        id	INTEGER NOT NULL,
                                        download_id	INTEGER NOT NULL,
                                        time	TEXT,
                                        etime	TEXT,
                                        day	TEXT,
                                        FOREIGN KEY(download_id) REFERENCES downloads(id),
                                        PRIMARY KEY(id)
                                    );";
        self.execute(sql_db);
        let sql_db="
                                    CREATE TABLE settings (
                                        id	INTEGER NOT NULL,
                                        name	TEXT NOT NULL UNIQUE,
                                        value	TEXT NOT NULL,
                                        PRIMARY KEY(id)
                                    );";
        self.execute(sql_db);
        let sql_db="
                                    INSERT INTO settings (id, name, value)
                                    VALUES (NULL, 'downloader_status', '1');
                                    ";
        self.execute(sql_db);
        let sql_db="
                                    INSERT INTO settings (id, name, value)
                                    VALUES (NULL, 'schedule_status', '1');
                                    ";
        self.execute(sql_db);
        
    }
}

pub fn get_sqlite()->sql_operations{
    sql_operations::new(ConnectionType::FILE, Some(Downloader_Settings::default().db_path()))
}
pub fn test_get_sqlite()->sql_operations{
    sql_operations::new(ConnectionType::FILE, Some(Downloader_Settings::default().testdb_path()))
}
#[macro_export]
macro_rules! sql_c {
    ($var:ident) => {
        let $var = get_sqlite();
    };
}
#[macro_export]
#[doc = r"

fn readall<T>(&self,limit:i32,sort_by:&str) -> Vec<Settings>

limit_order!(limit,sort_by,limit_query,order_by_query);

use i32 for limit 

use String for sort_by

use limit_query var name if you mixup with query

use order_by_query var name if you mixup with query 

 let mut get_id = sqlite
            .conn
            .prepare(&format!(
                
                    SELECT *
                    FROM settings
                    {}
                    {}
                    ;
        ,limit_query,order_by_query
            ))
            .unwrap();
"]
macro_rules! limit_order {
    ($limit:ident,$sort_by:ident,$limit_query:ident,$order_by_query:ident) => {
        let mut $limit_query = String::from("");
        let mut $order_by_query=String::from("");
        if $limit>0{
            $limit_query= format!("LIMIT {}",$limit).to_string();
        }
        if$ sort_by !=""{
            $order_by_query= format!("ORDER BY {} DESC",$sort_by).to_string();
        }
    };
}
#[macro_export]
macro_rules! test_sql_c {
    ($var:ident) => {
        let $var = test_get_sqlite();
    };
}
