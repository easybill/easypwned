use std::io;
use ::tokio::sync::mpsc::{Sender, Receiver};
use tokio::io::AsyncWriteExt;
use tokio::task::JoinHandle;
use crate::sink::SinkMsg;

struct SinkStdout {
    recv: Receiver<SinkMsg>
}

impl SinkStdout {
    pub fn spawn() -> (JoinHandle<()>, Sender<SinkMsg>) {

        let (sender, recv) = ::tokio::sync::mpsc::channel(1000);

        let jh = ::tokio::spawn(async move {
            (Self {recv}).run().await.expect("stdout sink crashed.");
        });

        (jh, sender)
    }

    pub async fn run(&mut self) -> Result<(), ()> {
        loop {
            match self.recv.recv().await {
                None => continue,
                Some(s) => match s {
                    SinkMsg::Finish => return Ok(()),
                    SinkMsg::Data(data, ok) => {
                        ::tokio::io::stdout().write_all(&data).await.expect("could not write to stout");
                    }
                }
            };
        }
    }
}