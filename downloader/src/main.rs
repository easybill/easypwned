use structopt::StructOpt;

use crate::download_coordinator::DownloadCoordinator;
use crate::downloader_http::DownloaderHttp;
use crate::sink::bloom::SinkBloom;
use crate::sink::stdout::SinkStdout;

pub mod download_coordinator;
pub mod downloader_http;
pub mod sink;

#[derive(Clone)]
pub struct DownloadConfig {
    pub opt: Opt,
    pub number_of_downloader: u32,
}


#[derive(StructOpt, Debug, Clone)]
#[structopt()]
pub struct Opt {
    #[structopt(long = "sink-bloom-file")]
    sink_bloom_file: Option<String>,
    #[structopt(long)]
    sink_stdout: bool,
    #[structopt(long)]
    debug_console_subscriber: bool,
    #[structopt(long = "parallel", default_value="60")]
    parallel: u32,
}

pub async fn download(config: DownloadConfig) {

    let (sinks_jhs, sinks_senders) = {
        let mut jhs = vec![];
        let mut senders = vec![];

        match config.opt.sink_stdout {
            true => {

                let (jh, sender) = SinkStdout::spawn();
                jhs.push(jh);
                senders.push(sender);
            },
            false => {}
        };

        match config.opt.sink_bloom_file {
            Some(ref _v) => {
                let (jh, sender) = SinkBloom::spawn(config.clone());
                jhs.push(jh);
                senders.push(sender);
            },
            None => {}
        };

        (jhs, senders)
    };

    if sinks_jhs.len() == 0 {
        eprintln!("you need to define a sink, try --sink_stdout");
        return;
    }

    let (_coordinator_jh, coordinator) = DownloadCoordinator::spawn(
        sinks_senders
    );

    for _i in 0..config.number_of_downloader {
        DownloaderHttp::spawn(coordinator.clone());
    }

    for jh in sinks_jhs {
        jh.await.expect("sink crashed");
        eprintln!("finish sink")
    }
}

#[tokio::main]
async fn main() -> ::anyhow::Result<(), ::anyhow::Error> {

    let opt: Opt = Opt::from_args();

    if opt.debug_console_subscriber {
        console_subscriber::init();
    }

    let download_config = DownloadConfig {
        number_of_downloader: opt.parallel,
        opt,
    };

    download(download_config).await;

    return Ok(());
}
