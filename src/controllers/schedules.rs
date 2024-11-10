use crate::components::downloader::{self, DownloadFile};
use crate::db::db_models::{Downloads, Schedules, CRUD};
use crate::global::Downloader_Settings;
use chrono::{DateTime, Datelike, NaiveDateTime, TimeZone, Utc};
use crate::controllers::downloader::{ready_to_download_queue};
pub fn get_schedules(limits: i32, sort_by: &str) -> String {
    let new_scheduler = Schedules::blank();
    let new_scheduler = new_scheduler.readall::<Schedules>(limits, sort_by);
    let json = serde_json::to_string(&new_scheduler).expect("Failed to serialize to JSON");
    json
}

pub fn add_new_schedules(download_id: i32, time: &str,etime:&str, day: &str) -> i32 {
    
    let downloader_blank: Downloads = Downloads::blank();
    let downloader_blank: Downloads = downloader_blank.read_by_id::<Downloads>(download_id);
    if downloader_blank.id != Some(0) {
        let mut new_scheduler = Schedules::blank();
        new_scheduler.download_id = download_id;
        new_scheduler.time = time.to_string();
        new_scheduler.etime = etime.to_string();
        new_scheduler.day = day.to_string();
        let id: i32 = new_scheduler.create::<Schedules>();
        return id;
    } else {
        return -1;
    }
}
pub fn active_current_schedules()->String{
    let mut schedules_blank: Schedules = Schedules::blank();
    let now: DateTime<Utc>=Utc::now();
    schedules_blank.day =now.weekday().num_days_from_monday().to_string();
    let schedules: Vec<Schedules> = schedules_blank.today_schedules();
    let time_format: String = Downloader_Settings::default().time_format();
    for scheduled in schedules{
        let etime: DateTime<Utc> = NaiveDateTime::parse_from_str(&scheduled.etime, &time_format).expect("cant change exchange end time strings from dbs").and_utc();
        let time: DateTime<Utc> = NaiveDateTime::parse_from_str(&scheduled.time, &time_format).expect("cant change exchange time strings from dbs").and_utc();
        if time<  now && now<etime {
            ready_to_download_queue(scheduled.download_id);
        }
    }

    String::from("")
}