use futures_util::future::ok;
use futures_util::StreamExt;
use reqwest::Client;
use reqwest::header::{CONTENT_LENGTH,RANGE};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::hash::Hash;
use std::io::{self, Read, Write};
use std::path::Path;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::thread;

use crate::controllers::settings::get_downloader_status;
use crate::db::db_models::{Downloads, CRUD};
pub struct DownloadFile{
    url: String, output_path: String,status:bool
}
impl DownloadFile {
    pub fn new(url: &str, output_path: &str,status:bool)->DownloadFile{
        DownloadFile{ url: url.to_string(), output_path: output_path.to_string(), status}
    }
    pub async fn download_file(&self) -> Result<i32, Box<dyn std::error::Error+Send>> {
        if !Path::new(&self.output_path).exists() {
            match  File::create(&self.output_path.clone()) {
                Ok(mut new_file) => match new_file.flush() {
                    Ok(_) => (),
                    Err(e) =>{
                        println!("cant flush the file");
                        println!("{}",e);
                        ()
                    }, 
                }
                Err(e) =>{
                    println!("cant create the file");
                    println!("{}",e);
                }
            }

        }
        let  file_ptr_resume_checker =File::open(&self.output_path.clone()).expect("cant read file");
        let file = BufReader::new(file_ptr_resume_checker.try_clone().expect("an error occured in resuming file"));
        let mut downloaded=0;
        for _byte in file.bytes() {
            downloaded+=1
        }
        println!("{}", downloaded);
        let client = Client::new();
        let response = client.get(&self.url).header(RANGE,format!("bytes={:?}-",downloaded)).send().await.unwrap();
        let headers=response.headers();
        // Check for content length
        let total_size = headers.get(CONTENT_LENGTH)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.parse::<u64>().ok())
            .unwrap_or(0);
       
        if total_size==0{
            println!("downloaded");
            return Err(Box::new(io::Error::new(io::ErrorKind::AlreadyExists, "File already downloaded")));
        }
        // current session downloaded for resumed downloads
        let mut session_downloaded=0;
        let mut file_ptr =OpenOptions::new().append(true).open(&self.output_path).expect("cant read file");
        // Stream the response body
        let mut content = response.bytes_stream();
        let output = self.output_path.clone();
        let mut blank = Downloads::blank();
        blank.output=output;
        while let Some(chunk) = content.next().await {
            if total_size > 0 && session_downloaded >= total_size{
                break;
            }
            let blank=blank.get_download_with_status_and_name();
            let downloader_status = get_downloader_status().parse::<i32>().unwrap();
            if downloader_status==0{
                return Ok(201);
            }
            if blank.status ==0{
                return Ok(201);
            }
            let chunk = chunk.expect("cant read byte chunks");
            let size = chunk.len() as u64;
            session_downloaded += size;
            downloaded+=size;

            // Write chunk to file
            let _ = file_ptr.write_all(&chunk);
    
            // Calculate and display progress
            let progress = (downloaded as f64 / total_size as f64) * 100.0;
        }
        blank.status=3;
        blank.update::<Downloads>();
        return Ok(200);
    }
    pub async fn get_total_size(&self)->u64{
        let client = Client::new();
        let response = client.get(&self.url).send().await.expect("maybe wrong url for downloader file");
        let headers=response.headers();
        // Check for content length
        let total_size = headers.get(CONTENT_LENGTH)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.parse::<u64>().ok())
            .unwrap_or(0);
        total_size
    }
}
