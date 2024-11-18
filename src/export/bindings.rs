
use crate::controllers::{downloader,schedules,settings};
use crate::db::db_connection::get_sqlite;
use crate::services::downloader::{main_downloader_service, stop_downloader_service};
use crate::sql_c;
use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use std::os::raw::c_int;
use std::path::Path;
use std::{env, fs, thread};
use directories::UserDirs;
#[no_mangle]
pub extern  "C" fn default_folder_c(path:*const c_char)->bool {
    
    if let Some(user_dirs) = UserDirs::new() {
        let download_path= user_dirs.download_dir().unwrap();
        match env::set_current_dir(download_path) {
            Ok(_) =>true,
            Err(_) =>false
        }
    }else{
        false
    }
    
}

#[no_mangle]
pub extern  "C" fn is_data_exists_c(path:*const c_char)->bool {
    let binding = unsafe {
        CStr::from_ptr(path).to_string_lossy().into_owned()
    }.clone();
    let download_path = binding.as_str();
    match Path::new(download_path).exists() {
        true => true,
        false => false,
    }
}

#[no_mangle]
pub extern  "C" fn change_download_folder_c(path:*const c_char){
    // make long live download path in safe way
    let binding = unsafe {
        assert!(!path.is_null());
        CStr::from_ptr(path).to_string_lossy().into_owned()
    }.clone();

    let download_path = binding.as_str();
    match fs::create_dir_all(download_path) {
        Ok(_) => println!("create download path"),
        Err(_) => println!("path exists.jump"),
    }
    match env::set_current_dir(download_path) {
        Ok(_) => println!("changed data path to {}",download_path),
        Err(_) =>print!("something went wrong.cant change data path to {}",download_path),
    }
}
#[no_mangle]
pub extern "C" fn get_downloads_c(limits: c_int, sort_by: *const c_char) -> *const c_char {
    let sort_by_str = unsafe { CStr::from_ptr(sort_by).to_str().unwrap_or("") };

    let result = downloader::get_downloads(limits as i32, sort_by_str);
    CString::new(result).unwrap().into_raw()
}
#[no_mangle]
pub extern "C" fn add_new_download_general_c(
    url: *const c_char, 
    output_path: *const c_char, 
) -> c_int {
    let url_str = unsafe {
        CStr::from_ptr(url).to_string_lossy().into_owned()
    };

    let output_path_str = unsafe {
        CStr::from_ptr(output_path).to_string_lossy().into_owned()
    };
    let types_str = CString::new("part".to_string()).to_owned().unwrap().to_string_lossy().into_owned();
    let version_str = CString::new("1.0.0".to_string()).to_owned().unwrap().to_string_lossy().into_owned();
    let category_str = CString::new("general".to_string()).to_owned().unwrap().to_string_lossy().into_owned();
    let divise_str = CString::new("general".to_string()).to_owned().unwrap().to_string_lossy().into_owned();

    let result = downloader::add_new_download(
        &url_str, 
        &output_path_str, 
        &types_str, 
        &version_str, 
        &category_str, 
        &divise_str
    );

    result
}

#[no_mangle]
pub extern "C" fn add_new_download_c(
    url: *const c_char, 
    output_path: *const c_char, 
    types: *const c_char, 
    version: *const c_char, 
    category: *const c_char, 
    divise: *const c_char
) -> c_int {

    let url_str = unsafe {
        CStr::from_ptr(url).to_string_lossy().into_owned()
    };

    let output_path_str = unsafe {
        CStr::from_ptr(output_path).to_string_lossy().into_owned()
    };

    let types_str = unsafe {
        CStr::from_ptr(types).to_string_lossy().into_owned()
    };

    let version_str = unsafe {
        CStr::from_ptr(version).to_string_lossy().into_owned()
    };

    let category_str = unsafe {
        CStr::from_ptr(category).to_string_lossy().into_owned()
    };

    let divise_str = unsafe {
        CStr::from_ptr(divise).to_string_lossy().into_owned()
    };

    // Now call the Rust function wreith the extracted parameters
    let result = downloader::add_new_download(
        &url_str, 
        &output_path_str, 
        &types_str, 
        &version_str, 
        &category_str, 
        &divise_str
    );
    result
}
#[no_mangle]
pub extern "C" fn ready_to_download_queue_c(id: c_int) -> bool {
    downloader::ready_to_download_queue(id as i32)
}
#[no_mangle]
pub extern "C" fn remove_download_by_id_c(id: c_int) -> bool {
    downloader::remove_download_by_id(id as i32)
}
#[no_mangle]
pub extern "C" fn remove_schedule_by_id_c(id: c_int) -> bool {
    schedules::remove_schedule_by_id(id as i32)
}
#[no_mangle]
pub extern "C" fn pause_download_by_id_c(id: c_int) {
    downloader::pause_download_by_id(id as i32);
}

#[no_mangle]
pub extern "C" fn get_file_info_c(id: c_int, output_path: *const c_char) -> *const c_char {
    let output_path_str = unsafe {
        CStr::from_ptr(output_path).to_string_lossy().into_owned()
    };

    let file_info = downloader::get_file_info(id as i32, &output_path_str);
    let json = serde_json::to_string(&file_info).expect("Failed to serialize to JSON");
    CString::new(json).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn get_schedules_c(limits: c_int, sort_by: *const c_char) -> *const c_char {
    let sort_by_str = unsafe {
        CStr::from_ptr(sort_by).to_string_lossy().into_owned()
    };

    let result = schedules::get_schedules(limits as i32, &sort_by_str);
    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn add_new_schedules_c(download_id: c_int, time: *const c_char, etime: *const c_char, day: *const c_char) -> c_int {
    let time_str = unsafe {
        CStr::from_ptr(time).to_string_lossy().into_owned()
    };

    let etime_str = unsafe {
        CStr::from_ptr(etime).to_string_lossy().into_owned()
    };

    let day_str = unsafe {
        CStr::from_ptr(day).to_string_lossy().into_owned()
    };

    schedules::add_new_schedules(download_id as i32, &time_str, &etime_str, &day_str)
}

#[no_mangle]
pub extern "C" fn active_current_schedules_c() -> *const c_char {
    let result = schedules::active_current_schedules();
    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn update_setting_c(key: *const c_char, value: *const c_char) {
    let key_str = unsafe {
        CStr::from_ptr(key).to_string_lossy().into_owned()
    };

    let value_str = unsafe {
        CStr::from_ptr(value).to_string_lossy().into_owned()
    };

    settings::update_setting(&key_str, &value_str);
}

#[no_mangle]
pub extern "C" fn get_setting_c(key: *const c_char) -> *const c_char {
    let key_str = unsafe {
        CStr::from_ptr(key).to_string_lossy().into_owned()
    };

    let result = settings::get_setting(&key_str);
    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn get_downloader_status_c() -> *const c_char {
    let result = settings::get_downloader_status();
    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn turn_downloader_status_c() -> *const c_char {
    let result = settings::turn_downloader_status();
    CString::new(result).unwrap().into_raw()
}
#[no_mangle]
pub extern "C" fn free_c_string(s: *const c_char) {
    if s.is_null() { return; }
    unsafe {
        let _ = CString::from_raw(s as *mut c_char);
    }
}
#[no_mangle]
pub extern  "C" fn init_db_c(){
    sql_c!(sqlite);
    sqlite.init_db();
}

#[no_mangle]
pub extern  "C" fn start_background_downloader_service_c(){
    thread::spawn(move ||{
        main_downloader_service();
    });   
}
#[no_mangle]
pub extern  "C" fn stop_background_downloader_service_c(){
    stop_downloader_service();
}