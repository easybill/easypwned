use structopt::StructOpt;
use crate::download_coordinator::DownloadCoordinator;
use crate::downloader_http::DownloaderHttp;

pub mod download_coordinator;
pub mod downloader_http;

pub struct DownloadConfig {
    pub number_of_downloader: u32,
}


#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct Opt {
    #[structopt()]
    sink_bloom_file: Option<String>,
    #[structopt()]
    sink_stdout: Option<String>,
}

pub async fn download(config: DownloadConfig) {

    let (coordinator_jh, coordinator) = DownloadCoordinator::spawn();

    for i in 1..config.number_of_downloader {
        DownloaderHttp::spawn(coordinator.clone());
    }

    coordinator_jh.await.expect("could not join");

}

#[tokio::main]
async fn main() -> ::anyhow::Result<(), ::anyhow::Error> {

    let opt: Opt = Opt::from_args();
    tracing_subscriber::fmt::init();

    println!("{:?}", opt);

    let download_config = DownloadConfig {
        number_of_downloader: 10,
    };

    download(download_config).await;

    return Ok(());
}
