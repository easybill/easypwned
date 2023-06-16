use structopt::StructOpt;
use crate::download_coordinator::DownloadCoordinator;
use crate::downloader_http::DownloaderHttp;

pub mod download_coordinator;
pub mod downloader_http;
pub mod sink_csv;

pub struct DownloadConfig {
    pub number_of_downloader: u32,
}


#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct Opt {
    #[structopt(long = "bloomfile", default_value = "easypwned.bloom")]
    bloomfile: String,
    #[structopt(long = "create_bloom_file_from_file")]
    create_bloom_file_from_file: Option<String>,
    #[structopt(long = "bind", default_value = "0.0.0.0:3342")]
    bind: String,
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
