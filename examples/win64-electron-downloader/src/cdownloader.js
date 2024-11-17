import React, { useEffect, useState } from 'react';

const Downloader = () => {
    const downloader = window.electron.downloader;
    const [url, setUrl] = useState('');
    const [outputPath, setOutputPath] = useState('');
    const [downloads, setDownloads] = useState([]);
    const [startTime, setStartTime] = useState(''); // For start time input
    const [endTime, setEndTime] = useState(''); // For end time input
    const [scheduleDay, setScheduleDay] = useState(''); // For the day of the week

    useEffect(() => {
        downloader.changeDownloaderFolder("C:\\Users\\wizif\\Downloads");
        downloader.firstDataInitialize();
        updateDownloads();
        handleStartService();
        console.log("Downloader initialized");
            setInterval(() => {
                updateDownloads();
            }, 1000);
        
    }, []);

    const updateDownloads = () => {
        const newDownloads = downloader.getDownloads(10, 'id');
        setDownloads(newDownloads);
    };

    const formatDateTime = (date) => {
        const day = String(date.getDate()).padStart(2, '0');
        const month = String(date.getMonth() + 1).padStart(2, '0'); // Months are 0-indexed
        const year = date.getFullYear();
        const hours = String(date.getHours()).padStart(2, '0');
        const minutes = String(date.getMinutes()).padStart(2, '0');
        return `${day}${month}${year}${hours}${minutes}`;
    };

    const handleAddDownload = () => {
        if (url && outputPath) {
            downloader.addNewDownload(url, outputPath);
            updateDownloads();
            setUrl('');
            setOutputPath('');
        } else {
            alert("Please provide both URL and Output Path.");
        }
    };

    const handleScheduleDownload = () => {
        if (url && outputPath && startTime && endTime && scheduleDay) {
            const downloadId = downloader.addNewDownload(url, outputPath);

            // Format start and end times
            const startTimeFormatted = formatDateTime(new Date(startTime));
            const endTimeFormatted = formatDateTime(new Date(endTime));

            // Pass the parameters to addNewSchedules
            downloader.addNewSchedules(downloadId, startTimeFormatted, endTimeFormatted, scheduleDay);
            updateDownloads();
            setUrl('');
            setOutputPath('');
            setStartTime('');
            setEndTime('');
            setScheduleDay('');
            alert("Download scheduled!");
        } else {
            alert("Please fill all fields to schedule a download.");
        }
    };

    const handleStartService = () => {
        downloader.startBackgroundDownloaderService();
        updateDownloads()
        console.log("Background downloader service started");
    };

    const handleStopService = () => {
        downloader.stopBackgroundDownloaderService();
    };

    const handleStartDownloadWithID = (id) => {
        downloader.readyToDownloadQueue(id);
        updateDownloads();
    };

    const handleStopDownloadWithID = (id) => {
        downloader.pauseDownloadById(id);
        updateDownloads();
    };

    return (
        <div>
            <h1>Downloader</h1>
            <input
                type="text"
                value={url}
                onChange={(e) => setUrl(e.target.value)}
                placeholder="Download URL"
            />
            <input
                type="text"
                value={outputPath}
                onChange={(e) => setOutputPath(e.target.value)}
                placeholder="Output Path"
            />
            <button onClick={handleAddDownload}>Add Download</button>

            <h2>Schedule a Download</h2>
            <input
                type="datetime-local"
                value={startTime}
                onChange={(e) => setStartTime(e.target.value)}
                placeholder="Start Time"
            />
            <input
                type="datetime-local"
                value={endTime}
                onChange={(e) => setEndTime(e.target.value)}
                placeholder="End Time"
            />
            <input
                type="text"
                value={scheduleDay}
                onChange={(e) => setScheduleDay(e.target.value)}
                placeholder="Schedule Day (e.g. 'Monday')"
            />
            <button onClick={handleScheduleDownload}>Schedule Download</button>
            {/* <button onClick={handleStartService}>Start Service</button> */}
            <button onClick={handleStopService}>Stop Downloader Service</button>

            <h2>Active Downloads</h2>
            <ul>
                {downloads.map((download, index) => {
                    const info = downloader.getFileInfo(download.id, download.output)
                    return(
                    <li key={index}>
                        <strong>ID:</strong> {download.id}<br />
                        <strong>URL:</strong> {download.url}<br />
                        <br />
                        <strong>Output:</strong> {download.output}<br />
                        <strong>progress:</strong> {parseInt(info.progress).toFixed(2)}%<br />
                        <strong>downloaded:</strong> {info.downloaded} from {info.size}
                        {download.status === 1 ? (
                            <button onClick={() => handleStopDownloadWithID(download.id)}>Pause</button>
                        ) : download.status === 0 ? (
                            <button onClick={() => handleStartDownloadWithID(download.id)}>Start</button>
                        ) : (
                            download.status === 2 ? <button onClick={() => handleStopDownloadWithID(download.id)}>Pause</button>:<button disabled>Downloaded</button>
                        )}
                    </li>
                )})}
            </ul>
        </div>
    );
};

export default Downloader;
