use crate::downloader_http::{DownloaderCommanderMsgRequest, DownloaderCommanderMsgWork};
use crate::sink::SinkMsg;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::task::JoinHandle;

const RANGES: u32 = 1024 * 1024;

pub struct DownloadCoordinator {
    sinks: Vec<Sender<SinkMsg>>,
    recv: Receiver<DownloaderCommanderMsgRequest>,
    current_range: u32,
    resolved_ranges: u32,
}

impl DownloadCoordinator {
    pub fn spawn(
        sinks: Vec<Sender<SinkMsg>>,
    ) -> (JoinHandle<()>, Sender<DownloaderCommanderMsgRequest>) {
        let (send, recv) = ::tokio::sync::mpsc::channel(10_000);

        let jh = ::tokio::spawn(async move {
            (Self {
                recv,
                current_range: 0,
                resolved_ranges: 0,
                sinks,
            })
            .run()
            .await;
        });

        (jh, send)
    }

    pub async fn run(&mut self) {
        loop {
            let msg = match self.recv.recv().await {
                Some(s) => s,
                None => continue,
            };

            match msg {
                DownloaderCommanderMsgRequest::AskForWork(sender) => {
                    let msg_work = if self.current_range <= RANGES - 1 {
                        self.current_range += 1;
                        Some(DownloaderCommanderMsgWork {
                            range: self.current_range - 1,
                        })
                    } else {
                        None
                    };

                    sender.send(msg_work).expect("could not send work");
                }
                DownloaderCommanderMsgRequest::SendWork(w) => {
                    for sink in &self.sinks {
                        sink.send(SinkMsg::Data(w.prefix.clone(), w.bytes.clone()))
                            .await
                            .expect("sink was killed");
                    }

                    if self.resolved_ranges % 1000 == 0 {
                        eprintln!(
                            "{}/{} - {}%",
                            self.resolved_ranges,
                            RANGES,
                            self.resolved_ranges as f64 / RANGES as f64 * 100.0
                        );
                    }

                    self.resolved_ranges += 1;

                    if self.resolved_ranges >= RANGES {
                        for sink in &self.sinks {
                            match sink.send(SinkMsg::Finish).await {
                                Ok(_) => {}
                                Err(_e) => {}
                            };
                        }
                    }
                }
            };
        }
    }
}
