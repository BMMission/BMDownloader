
#[cfg(test)]
mod tests {
    use crate::components::downloader::DownloadFile;
    use crate::db::db_connection::get_sqlite;
    use crate::db::db_models::{Downloads, Schedules, Settings, CRUD};
    use crate::{sql_c, test_sql_c};
    
 
    use tokio::task;
    #[doc = r"this function a wrapper to sql_c macro to run init_db across tests"]
    #[deprecated="use if you clone for first time and dont have dbs for tests"]
    #[test]
    fn init_db(){
        sql_c!(conn);
        conn.init_db();
    }
    #[tokio::test]
    #[doc = r"general purpose test for download files"]
    async fn test_downloader() {
        // let url1 = "https://sample-videos.com/video321/mp4/720/big_buck_bunny_720p_2mb.mp4";
        // let output_path1 = "big_buck_bunny_720p_2mb.mp4";
        // let url2 = "https://sample-videos.com/text/Sample-text-file-1000kb.txt";
        // let output_path2 = "Sample-text-file-1000kb.txt";

        // // Spawn tasks for both downloads
        // let download1 = task::spawn(download_file(url1, output_path1));
        // let download2 = task::spawn(download_file(url2, output_path2));

        // // Await both downloads to complete
        // let (result1, result2) = tokio::join!(download1, download2);
        // let result1 = result1.unwrap();
        // let result2 = result2.unwrap();
        // // Assert the results
        // assert!(result1.is_ok(),"file_exists");
        // assert!(result2.is_ok(),"file_exists");

    }
    #[test]
    #[doc = "test sqlite connection"]
    fn test_1_layer_(){
        init_db();
    }
    #[test]
    
    #[doc = "test readall_downloads into db"]
    fn test_1_layer__readall_downloads(){
        let download: Downloads=Downloads::new(None, 0, String::new(), String::new(), 0);
        download.readall::<Downloads>(0,"id");

     }
    #[test]
    
    #[doc = "test insert_downloads into db"]
    fn test_1_layer__insert_downloads(){
        let download=Downloads::new(None, 0, "test_1_layer__insert_downloads.txt".to_owned(), "google.com".to_owned(), 11);
        download.create::<Downloads>();
     }
     #[test]
     
    #[doc = "test read_by_id_downloads from db"]
    fn test_1_layer__read_by_id_downloads(){
        let download: Downloads=Downloads::new(None ,0, String::new(), String::new(), 0);
        
     }
     #[test]
     
    #[doc = "test update_downloads into db"]
    fn test_1_layer__update_downloads(){
        let download: Downloads=Downloads::new(None ,1, String::new(), String::new(), 0);
        let mut download: Downloads = download.read_by_id::<Downloads>(2);
        download.output="testcase output update".to_string();
     }
     #[test]
     
     #[doc = "test insert_schedules into db"]
     fn test_1_layer__zinsert_schedules(){
         let mut data=Schedules::blank();
         data.download_id = 1;
         let test_1:i32  = data.create::<Schedules>();
      }
      #[test]
      
      #[doc = "test readall_schedules from db"]
     fn test_1_layer__zreadall_schedules(){
         let data=Schedules::blank();
         let test_1  = data.readall::<Schedules>(0,"id");
      }
      #[test]
      
      #[doc = "test read_by_id_schedules from db"]
      fn test_1_layer__zread_by_id_schedules(){
        let data=Schedules::blank();

        let test_1  = data.read_by_id::<Schedules>(1);

       }
       #[test]
       
       #[doc = "test update_schedules into db"]
       fn test_1_layer__zupdate_schedules(){
         let data=Schedules::blank();
         let mut data  = data.read_by_id::<Schedules>(1);
         data.day="testcase day update".to_string();
         let test_1  = data.update::<Schedules>();
        }


    #[test]
    
     #[doc = "test insert_settings into db"]
     fn test_1_layer__insert_settings(){
         let data=Settings::blank();
         let test_1:i32  = data.create::<Settings>();
      }
      #[test]
      
      #[doc = "test readall_settings from db"]
     fn test_1_layer__readall_settings(){
         let data=Settings::blank();
         let test_1  = data.readall::<Settings>(0,"id");
      }
      #[test]
      
      #[doc = "test read_by_id_settings from db"]
      fn test_1_layer__read_by_id_settings(){
        let data=Settings::blank();
        let test_1  = data.read_by_id::<Settings>(1);

       }
       #[test]
       
       #[doc = "test update_downloads into db"]
       fn test_1_layer__update_settings(){
         let data=Settings::blank();
         let mut data  = data.read_by_id::<Settings>(1);
         data.name="testcase name update".to_string();
         data.value="testcase value update".to_string();
         let test_1  = data.update::<Settings>();
        }
    
}
