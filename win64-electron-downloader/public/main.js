// import { app, BrowserWindow } from 'electron';
// const {BrowserWindow} = require('electron');
// const {app} = require('electron');
const { app, BrowserWindow } = require('electron');
const path = require('path');
const downloader =  require("../src/downloader.js");
// import * as downloader from "../src/downloader.js";

require('@electron/remote/main').initialize()
// exports.downloader = downloader;
function createWindow() {
    const mainWindow = new BrowserWindow({
        sandbox: false,
        width: 800,
        height: 600,
        webPreferences: {
            preload: path.join(__dirname, "../preload.js"),
            contextIsolation: true,
            nodeIntegration: true

        },
    });
    mainWindow.loadFile('../dist/index.html');
    require('@electron/remote/main').enable(mainWindow.webContents);
    mainWindow.webContents.openDevTools();
}

app.whenReady().then(createWindow);

app.on('window-all-closed', () => {
    if (process.platform !== 'darwin') {
        app.quit();
    }
});

app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) {
        createWindow();
    }
});