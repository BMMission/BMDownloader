const { contextBridge } = require('electron');
const { isCompositeComponent } = require('react-dom/test-utils');
const downloader = require("./src/downloader.js");
contextBridge.exposeInMainWorld('electron', {
    downloader:downloader,
});