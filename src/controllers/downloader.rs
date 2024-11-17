use crate::components::downloader::{self, DownloadFile};
use crate::db::db_models::{Downloads, CRUD};
use std::collections::HashMap;
use std::fs::metadata;
use std::path::Path;

pub fn get_downloads(limits: i32, sort_by: &str) -> String {
    let new_downloader = Downloads::blank();
    let all_downloads = new_downloader.readall::<Downloads>(limits, sort_by);
    let json = serde_json::to_string(&all_downloads).expect("Failed to serialize to JSON");
    json
}

pub fn add_new_download(
    url: &str,
    output_path: &str,
    types: &str,
    version: &str,
    category: &str,
    divise: &str,
) -> i32 {
    let mut new_downloader = Downloads::blank();
    new_downloader.url = url.to_string();
    new_downloader.output = output_path.to_string();
    new_downloader.types = types.to_string();
    new_downloader.version = version.to_string();
    new_downloader.category = category.to_string();
    new_downloader.divise = divise.to_string();

    new_downloader.id = Some(new_downloader.create::<Downloads>());

    let download_file_obj: DownloadFile =
        DownloadFile::new(&new_downloader.url, &new_downloader.output, false);
    let runtime: tokio::runtime::Runtime = tokio::runtime::Runtime::new().unwrap();
    let file_size: u64 = runtime.block_on(download_file_obj.get_total_size());
    new_downloader.file_size = file_size;
    new_downloader.update::<Downloads>();

    return new_downloader.id.unwrap();
}

pub fn ready_to_download_queue(id: i32) -> bool {
    let mut new_downloader: Downloads = Downloads::blank();
    new_downloader = new_downloader.read_by_id::<Downloads>(id);

    if new_downloader.status == 0 {
        new_downloader.status = 1;
        new_downloader.update::<Downloads>();
        true
    } else if new_downloader.status == 2 {
        new_downloader.status = 1;
        new_downloader.update::<Downloads>();
        true
    } else {
        false
    }
}

pub fn pause_download_by_id(id: i32) {
    let mut pause_download: Downloads = Downloads::blank();
    pause_download = pause_download.read_by_id::<Downloads>(id);
    pause_download.status = 0;
    pause_download.update::<Downloads>();
}

pub fn get_file_info(id: i32, output_path: &str) -> HashMap<String, String> {
    if !Path::new(&output_path).exists() {
        let mut file_info: HashMap<String, String> = HashMap::new();
        file_info.insert("downloaded".to_string(), 0.to_string());
        file_info.insert("size".to_string(), 0.to_string());
        file_info.insert("progress".to_string(), 0.to_string());
        return file_info;
    }

    let new_downloader: Downloads = Downloads::blank();
    let new_downloader: Downloads = new_downloader.read_by_id::<Downloads>(id);

    let metadata: std::fs::Metadata = metadata(output_path)
        .expect("File not found or some error occurred while accessing the file.");
    let file_size: u64 = metadata.len();

    let mut file_info: HashMap<String, String> = HashMap::new();
    let progress: f64 = (file_size as f64 / new_downloader.file_size as f64) * 100.0;

    file_info.insert("downloaded".to_string(), file_size.to_string());
    file_info.insert("size".to_string(), new_downloader.file_size.to_string());
    file_info.insert("progress".to_string(), progress.to_string());

    // Include the new fields in the response (if needed)
    file_info.insert("types".to_string(), new_downloader.types);
    file_info.insert("version".to_string(), new_downloader.version);
    file_info.insert("category".to_string(), new_downloader.category);
    file_info.insert("divise".to_string(), new_downloader.divise);

    file_info
}
