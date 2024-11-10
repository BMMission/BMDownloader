use std::{ thread::{self, sleep}, time::Duration};

use crate::{components::downloader::DownloadFile, controllers::{schedules::active_current_schedules, settings::{get_downloader_status, get_schedule_status}}, db::db_models::{Downloads, Schedules, CRUD}};
use crate::controllers::settings::{update_setting,get_setting};

#[doc = "this function listener loop until some downloads file get status 1"]
#[doc = "need to change to 2 that means we get into downloading mode"]
#[doc = "need to change to 0 that means we get into pause mode"]
pub fn downloader_queue(){
    
    let ready_for_dl = Downloads::get_active_downloads();
    let active_download = Downloads::blank();
    for id in ready_for_dl{
        let mut active_download = active_download.read_by_id::<Downloads>(id);
        println!("{:?}", active_download);
        active_download.status=2;
        active_download.update::<Downloads>();
        let url = active_download.url;
        let output = active_download.output;
        thread::spawn(move ||{

            let _download_status =downloader_service_loop(
                DownloadFile::new(
                    &url,
                    &output,
                    true), id
            );
            
    });
    
    }
}

pub fn downloader_service_loop(download_file:DownloadFile,id:i32)->i32{
    
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let result = runtime.block_on(download_file.download_file());
        match result {
            Ok(status) => {
                if status==200{
                    println!("Download completed with status: {}", status);
                    return 1;
                }
                else if status==202 {
                    let mut active_download = Downloads::blank();
                    active_download = active_download.read_by_id::<Downloads>(id);
                    active_download.status=0;
                    println!("Download paused with status: {}", status);
                    return 0;
                }else{
                    return -3;
                }
                
            }
            Err(e) => {
                eprintln!("Download failed: {}", e);
                return -1;
            }
        }

    
}

pub fn main_downloader_service(){
    update_setting("downloader_status", "1");
    Downloads::set_force_exit_to_active_downloads();
    loop {
        let setting_checker = get_downloader_status();
        
        if setting_checker ==1.to_string(){
            let schedule_checker = get_schedule_status();
            if schedule_checker == 1.to_string(){
                active_current_schedules();
            }
            downloader_queue();

            let _ = sleep(Duration::new(1,0));
           
            continue;
            
        }else if setting_checker =="0".to_string() {
            let _ = sleep(Duration::new(1,0));
            continue;
        }

    }
}
pub fn stop_downloader_service(){
    update_setting("status", "0");
}