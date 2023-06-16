use tokio::sync::mpsc::{Receiver, Sender};
use tokio::task::JoinHandle;
use crate::downloader_http::{DownloaderCommanderMsgRequest, DownloaderCommanderMsgWork};

const RANGES : u32 = 1024*1024;

pub struct DownloadCoordinator {
    recv: Receiver<DownloaderCommanderMsgRequest>,
    current_range: u32,
    resolved_ranges: u32,
}

impl DownloadCoordinator {
    pub fn spawn() -> (JoinHandle<()>, Sender<DownloaderCommanderMsgRequest>) {

        let (send, recv) = ::tokio::sync::mpsc::channel(10000);

        let jh = ::tokio::spawn(async move {
            (Self {
                recv,
                current_range: 0,
                resolved_ranges: 0,
            }).run().await;
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
                        Some(DownloaderCommanderMsgWork { range: self.current_range -1})
                    } else {
                        None
                    };

                    sender.send(msg_work).expect("could not send work");
                },
                DownloaderCommanderMsgRequest::SendWork(w) => {

                    println!("{}", String::from_utf8_lossy(&w.bytes));

                    if self.resolved_ranges % 1000 == 0 {
                        println!("{}/{} - {}%", self.resolved_ranges, RANGES, self.resolved_ranges as f64 / RANGES as f64 * 100.0);
                    }

                    self.resolved_ranges += 1;

                    if self.resolved_ranges == RANGES {
                        println!("all done");
                        break;
                    }
                }
            };
        }
    }
}