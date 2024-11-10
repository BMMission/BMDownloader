pub struct Downloader_Settings{
    sqlite_path:String,
    test_sqlite_path:String,
    time_format:String
}
impl Downloader_Settings {
    pub fn default()->Self{
        Self{
            sqlite_path:"bmm.db".to_string(),
            test_sqlite_path:"bmm.db".to_string(),
            time_format:"%d%m%Y%H%M".to_string()
        }
    }
    pub fn db_path(&self)->String{
        self.sqlite_path.clone()
    }
    pub fn testdb_path(&self)->String{
        self.sqlite_path.clone()
    }
    pub fn time_format(&self)->String{
        self.time_format.clone()
    }
}