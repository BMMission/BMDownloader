use crate::db::db_models::{Settings, CRUD};
pub fn update_setting(key:&str,value:&str){
    let mut setting_blank = Settings::find_by_key(key);
    setting_blank.value=value.to_string();
    setting_blank.update::<Settings>();
}
pub fn get_setting(key:&str)->String{
    return Settings::find_by_key(key).value;
}
pub fn get_downloader_status()->String{
    return Settings::find_by_key("downloader_status").value;
}
pub fn turn_downloader_status()->String{
    let mut setting_blank = Settings::find_by_key("downloader_status");
    if  setting_blank.value== "1".to_string(){
        setting_blank.value=0.to_string();
    }else {
        setting_blank.value=1.to_string();
    }
    setting_blank.update::<Settings>();
    setting_blank.value
    
}
pub fn active_schedule()->String{
    let mut setting_blank: Settings = Settings::find_by_key("schedule_status");
    setting_blank.value = "1".to_string();
    setting_blank.update::<Settings>();
    setting_blank.value
}
pub fn get_schedule_status()->String{
    let setting_blank: Settings = Settings::find_by_key("schedule_status");
    return setting_blank.value
}