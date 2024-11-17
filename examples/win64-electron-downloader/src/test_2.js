import {
    addNewDownload,
    changeDownloaderFolder,
    getDownloads,
    getFileInfo,
    readyToDownloadQueue
} from './downloader.js';

changeDownloaderFolder("C:\\Users\\wizif\\Downloads");
console.log(getFileInfo(15,"ddd.rar"))
// 
// addNewDownload("https://cdn.0game.ir/games/WWF Raw_0Game.rar","ddd.rar")

// let all_downloads = getDownloads(10,"id");
// // console.log(all_downloads)
// let download_now = all_downloads[0]
// readyToDownloadQueue(download_now.id)