import * as downloader from './downloader.cjs';


downloader.changeDownloaderFolder("/home/wizif/projects/oh_well/");

// changeDownloaderFolder("/home/wizif/projects/oh_well");
downloader.startBackgroundDownloaderService()
// downloader.firstDataInitialize();
let id = downloader.addNewGeneralDownload("https://sample-videos.com/video321/mp4/720/big_buck_bunny_720p_2mb.mp4","big_buck_bunny_720p_2mb.mp4")

// addNewDownload("https://sample-videos.com/video321/mp4/720/big_buck_bunny_720p_2mb.mp4","big_buck_bunny_720p_2mb_2.mp4")
// addNewDownload("https://sample-videos.com/video321/mp4/720/big_buck_bunny_720p_2mb.mp4","big_buck_bunny_720p_2mb_3.mp4")
// let all_downloads = getDownloads(10,"id");
// let download_now = all_downloads.reverse()[0]
// let download_now_1 = all_downloads.reverse()[1]
// let download_now_2 = all_downloads.reverse()[2]

// console.log(download_now)

// readyToDownloadQueue(download_now_1.id)
// readyToDownloadQueue(download_now_2.id)
console.log(downloader.getDownloads(10,"id").reverse())
console.log(id,downloader.removeDownloadByID(id))
setInterval(() => {

    // console.log('event loop is running downloader status is ',parseInt(getDownloaderStatus()) ?  true : false);
}, 1000);

