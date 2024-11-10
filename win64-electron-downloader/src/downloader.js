const { app } = require('electron');
var ffi = require('ffi-napi');
var ref = require('ref-napi')
// Define the C types you'll use
const cStringPtr = ref.refType(ref.types.char);
const cInt = ref.types.int;

// Load the Rust shared library
const rustLib = ffi.Library('public/bmdownloader.dll', {
    'get_downloads_c': [cStringPtr, [cInt, cStringPtr]],
    'add_new_download_c': [cInt, [cStringPtr, cStringPtr]],
    'ready_to_download_queue_c': [ref.types.bool, [cInt]],
    'is_data_exists_c': [ref.types.bool, [cStringPtr]],
    'pause_download_by_id_c': ['void', [cInt]],
    'get_file_info_c': [cStringPtr, [cInt, cStringPtr]],
    'get_schedules_c': [cStringPtr, [cInt, cStringPtr]],
    'add_new_schedules_c': [cInt, [cInt, cStringPtr, cStringPtr, cStringPtr]],
    'active_current_schedules_c': [cStringPtr, []],
    'update_setting_c': ['void', [cStringPtr, cStringPtr]],
    'get_setting_c': [cStringPtr, [cStringPtr]],
    'get_downloader_status_c': [cStringPtr, []],
    'turn_downloader_status_c': [cStringPtr, []],
    'change_download_folder_c': ['void', [cStringPtr]],
    'init_db_c': ['void', []],
    'stop_background_downloader_service_c': ['void', []],
    'start_background_downloader_service_c': ['void', []],
    'free_c_string': ['void', [cStringPtr]]
});
stringToBuffer=(str)=> {
    return Buffer.from(str);
}
exports.getDownloads=(limits, sortBy)=> {
    const sortByCStr = Buffer.from(sortBy);
    const resultPtr = rustLib.get_downloads_c(limits, sortByCStr);
    const result = JSON.parse(resultPtr.readCString());
    rustLib.free_c_string(resultPtr);
    return result;
}

exports.addNewDownload=(url, outputPath)=> {
    const urlCStr = stringToBuffer(url);
    const outputPathCStr = stringToBuffer(outputPath);
    return rustLib.add_new_download_c(urlCStr, outputPathCStr);
}

exports.readyToDownloadQueue=(id)=> {
    return rustLib.ready_to_download_queue_c(id);
}

exports.pauseDownloadById=(id)=> {
    rustLib.pause_download_by_id_c(id);
}

exports.getFileInfo=(id, outputPath)=> {
    const outputPathCStr = stringToBuffer(outputPath);
    const resultPtr = rustLib.get_file_info_c(id, outputPathCStr);
    const result =JSON.parse(resultPtr.readCString());
    rustLib.free_c_string(resultPtr);
    return result;
}

exports.getSchedules=(limits, sortBy)=> {
    const sortByCStr = stringToBuffer(sortBy);
    const resultPtr = rustLib.get_schedules_c(limits, sortByCStr);
    const result = JSON.parse(resultPtr.readCString());
    rustLib.free_c_string(resultPtr);
    return result;
}

exports.addNewSchedules=(downloadId, time, etime, day)=> {
    const timeCStr = stringToBuffer(time);
    const etimeCStr = stringToBuffer(etime);
    const dayCStr = stringToBuffer(day);
    return rustLib.add_new_schedules_c(downloadId, timeCStr, etimeCStr, dayCStr);
}

exports.activeCurrentSchedules=()=> {
    const resultPtr = rustLib.active_current_schedules_c();
    const result = resultPtr.readCString();
    rustLib.free_c_string(resultPtr);
    return result;
}

exports.updateSetting=(key, value)=> {
    const keyCStr = stringToBuffer(key);
    const valueCStr = stringToBuffer(value);
    rustLib.update_setting_c(keyCStr, valueCStr);
}

exports.getSetting=(key)=> {
    const keyCStr = stringToBuffer(key);
    const resultPtr = rustLib.get_setting_c(keyCStr);
    const result = resultPtr.readCString();
    rustLib.free_c_string(resultPtr);
    return result;
}

exports.getDownloaderStatus=()=> {
    const resultPtr = rustLib.get_downloader_status_c();
    const result = resultPtr.readCString();
    rustLib.free_c_string(resultPtr);
    return result;
}

exports.turnDownloaderStatus=()=> {
    const resultPtr = rustLib.turn_downloader_status_c();
    const result = resultPtr.readCString();
    rustLib.free_c_string(resultPtr);
    return result;
}
exports.changeDownloaderFolder=(download_path)=> {
    
    const download_pathCStr = stringToBuffer(download_path);
    rustLib.change_download_folder_c(download_pathCStr);
}
exports.isDataExists=(download_path)=> {
    
    const download_pathCStr = stringToBuffer(download_path);
    rustLib.is_data_exists_c(download_pathCStr);
}
exports.firstDataInitialize=()=> {
    rustLib.init_db_c();
}

exports.stopBackgroundDownloaderService=()=> {
    rustLib.stop_background_downloader_service_c();
}
exports.startBackgroundDownloaderService=()=>{
    rustLib.start_background_downloader_service_c();
}
