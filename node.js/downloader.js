import ffi from 'ffi-napi';
import ref from 'ref-napi';

// Define the C types you'll use
const cStringPtr = ref.refType(ref.types.char);
const cInt = ref.types.int;

// Load the Rust shared library
const rustLib = ffi.Library('./bmdownloader.dll', {
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
export function stringToBuffer(str) {
    return Buffer.from(str);
}
export function getDownloads(limits, sortBy) {
    const sortByCStr = Buffer.from(sortBy);
    const resultPtr = rustLib.get_downloads_c(limits, sortByCStr);
    const result = JSON.parse(resultPtr.readCString());
    rustLib.free_c_string(resultPtr);
    return result;
}

export function addNewDownload(url, outputPath) {
    const urlCStr = stringToBuffer(url);
    const outputPathCStr = stringToBuffer(outputPath);
    return rustLib.add_new_download_c(urlCStr, outputPathCStr);
}

export function readyToDownloadQueue(id) {
    return rustLib.ready_to_download_queue_c(id);
}

export function pauseDownloadById(id) {
    rustLib.pause_download_by_id_c(id);
}

export function getFileInfo(id, outputPath) {
    const outputPathCStr = stringToBuffer(outputPath);
    const resultPtr = rustLib.get_file_info_c(id, outputPathCStr);
    const result =JSON.parse(resultPtr.readCString());
    rustLib.free_c_string(resultPtr);
    return result;
}

export function getSchedules(limits, sortBy) {
    const sortByCStr = stringToBuffer(sortBy);
    const resultPtr = rustLib.get_schedules_c(limits, sortByCStr);
    const result = JSON.parse(resultPtr.readCString());
    rustLib.free_c_string(resultPtr);
    return result;
}

export function addNewSchedules(downloadId, time, etime, day) {
    const timeCStr = stringToBuffer(time);
    const etimeCStr = stringToBuffer(etime);
    const dayCStr = stringToBuffer(day);
    return rustLib.add_new_schedules_c(downloadId, timeCStr, etimeCStr, dayCStr);
}

export function activeCurrentSchedules() {
    const resultPtr = rustLib.active_current_schedules_c();
    const result = resultPtr.readCString();
    rustLib.free_c_string(resultPtr);
    return result;
}

export function updateSetting(key, value) {
    const keyCStr = stringToBuffer(key);
    const valueCStr = stringToBuffer(value);
    rustLib.update_setting_c(keyCStr, valueCStr);
}

export function getSetting(key) {
    const keyCStr = stringToBuffer(key);
    const resultPtr = rustLib.get_setting_c(keyCStr);
    const result = resultPtr.readCString();
    rustLib.free_c_string(resultPtr);
    return result;
}

export function getDownloaderStatus() {
    const resultPtr = rustLib.get_downloader_status_c();
    const result = resultPtr.readCString();
    rustLib.free_c_string(resultPtr);
    return result;
}

export function turnDownloaderStatus() {
    const resultPtr = rustLib.turn_downloader_status_c();
    const result = resultPtr.readCString();
    rustLib.free_c_string(resultPtr);
    return result;
}
export function changeDownloaderFolder(download_path) {
    
    const download_pathCStr = stringToBuffer(download_path);
    rustLib.change_download_folder_c(download_pathCStr);
}
export function isDataExists(download_path) {
    
    const download_pathCStr = stringToBuffer(download_path);
    rustLib.is_data_exists_c(download_pathCStr);
}
export function firstDataInitialize() {
    rustLib.init_db_c();
}

export function stopBackgroundDownloaderService() {
    rustLib.stop_background_downloader_service_c();
}
export function startBackgroundDownloaderService(){
    rustLib.start_background_downloader_service_c();
}
