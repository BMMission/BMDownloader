#[cfg(test)]
mod service_test{
    use crate::services::downloader::{main_downloader_service, stop_downloader_service};
    // #[test]
    #[doc = "please uncomment test if you realy know how main downloader service works"]
    fn test_downloader_service(){
        main_downloader_service();
    }
    fn test_pause_downloader_service(){
        stop_downloader_service();
    }
}