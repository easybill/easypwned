use crate::downloader::download_coordinator::DownloadCoordinator;
use crate::downloader::downloader_http::DownloaderHttp;

pub mod download_coordinator;
pub mod downloader_http;
pub mod sink_csv;

pub struct DownloadConfig {
    number_of_downloader: u32,
}

pub struct DownloaderHandle {

}

pub async fn download() {

    let (coordinator_jh, coordinator) = DownloadCoordinator::spawn();
    DownloaderHttp::spawn(coordinator.clone());
    DownloaderHttp::spawn(coordinator.clone());

    coordinator_jh.await.expect("could not join");

}