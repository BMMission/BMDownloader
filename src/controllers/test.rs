#[cfg(test)]
mod test_controller {
    use chrono::{Datelike, Duration, Utc};
    use crate::{
        controllers::{
            downloader::*,
            schedules::{active_current_schedules, add_new_schedules, get_schedules},
            settings::{get_downloader_status, turn_downloader_status, update_setting}
        },
        db::db_models::Settings,
        global::Downloader_Settings
    };

    #[test]
    fn test_controller_add_new_download() {
        let url: &str = "https://sample-videos.com/video321/mp4/720/big_buck_bunny_720p_2mb.mp4";
        let output: &str = "big_buck_bunny_720p_2mb.mp4";
        let types: &str = "video";
        let version: &str = "1.0";
        let category: &str = "movies";
        let divise: &str = "android";

        println!("New download added to queue with ID: {}", add_new_download(url, output, types, version, category, divise));

        let url: &str = "https://sample-videos.com/text/Sample-text-file-1000kb.txt";
        let output: &str = "Sample-text-file-1000kb.txt";
        let types: &str = "document";
        let version: &str = "1.1";
        let category: &str = "text_files";
        let divise: &str = "ios";

        println!("New download added to queue with ID: {}", add_new_download(url, output, types, version, category, divise));
    }

    #[test]
    fn test_controller_get_download() {
        println!("{}", get_downloads(10, "status"));
    }

    #[test]
    fn test_controller_add_new_with_start() {
        let url: &str = "https://sample-videos.com/video321/mp4/720/big_buck_bunny_720p_2mb.mp4";
        let output: &str = "big_buck_bunny_720p_2mb.mp4";
        let types: &str = "video";
        let version: &str = "1.0";
        let category: &str = "movies";
        let divise: &str = "android";
        let id: i32 = add_new_download(url, output, types, version, category, divise);
        ready_to_download_queue(id);

        let url: &str = "https://sample-videos.com/text/Sample-text-file-1000kb.txt";
        let output: &str = "Sample-text-file-1000kb.txt";
        let types: &str = "document";
        let version: &str = "1.1";
        let category: &str = "text_files";
        let divise: &str = "ios";
        let id: i32 = add_new_download(url, output, types, version, category, divise);
        ready_to_download_queue(id);
    }

    #[test]
    fn test_controller_pause_download_by_id() {
        pause_download_by_id(1);
        pause_download_by_id(2);
    }

    #[test]
    fn test_controller_play_download_by_id() {
        ready_to_download_queue(1);
        ready_to_download_queue(2);
    }

    #[test]
    fn test_controller_update_setting() {
        update_setting("status", "0");
    }

    #[test]
    fn test_controller_get_downloader_status_setting() {
        println!("Downloader service status is {}", get_downloader_status());
    }

    #[test]
    fn test_controller_turn_downloader_status() {
        println!("Downloader service status set to {}", turn_downloader_status());
    }

    #[test]
    fn test_controller_get_schedules() {
        println!("All schedules: \n{}", get_schedules(0, "id"));
    }

    #[test]
    fn test_controller_add_new_schedules() {
        let time_format: String = Downloader_Settings::default().time_format();
        let now: chrono::DateTime<Utc> = Utc::now();
        let today: String = now.weekday().num_days_from_monday().to_string();
        let six_hour_later = now + Duration::hours(6);
        let six_hour_later: String = format!("{}", six_hour_later.format(&time_format));
        let now: String = format!("{}", now.format(&time_format));

        println!("Schedule set with ID {}", add_new_schedules(1, &now, &six_hour_later, &today));
    }

    #[test]
    fn test_controller_active_current_schedules() {
        println!("Starting schedules for these downloads: \n{}", active_current_schedules());
    }
}
