use crate::downloader::download_coordinator::DownloadCoordinator;
use crate::downloader::downloader_http::DownloaderHttp;

pub mod download_coordinator;
pub mod downloader_http;
pub mod sink_csv;

pub struct DownloadConfig {
    pub number_of_downloader: u32,
}

pub struct DownloaderHandle {

}

pub async fn download(config: DownloadConfig) {

    let (coordinator_jh, coordinator) = DownloadCoordinator::spawn();

    for i in 1..config.number_of_downloader {
        DownloaderHttp::spawn(coordinator.clone());
    }

    coordinator_jh.await.expect("could not join");

}