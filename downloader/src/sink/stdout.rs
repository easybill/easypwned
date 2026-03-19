use crate::sink::SinkMsg;
use ::tokio::sync::mpsc::{Receiver, Sender};
use tokio::io::AsyncWriteExt;
use tokio::task::JoinHandle;

pub struct SinkStdout {
    recv: Receiver<SinkMsg>,
}

impl SinkStdout {
    pub fn spawn() -> (JoinHandle<()>, Sender<SinkMsg>) {
        let (sender, recv) = ::tokio::sync::mpsc::channel(1000);

        let jh = ::tokio::spawn(async move {
            (Self { recv }).run().await.expect("stdout sink crashed.");
        });

        (jh, sender)
    }

    pub async fn run(&mut self) -> Result<(), ()> {
        let mut stdout = ::tokio::io::stdout();

        loop {
            match self.recv.recv().await {
                None => continue,
                Some(s) => match s {
                    SinkMsg::Finish => return Ok(()),
                    SinkMsg::Data(prefix, data) => {
                        let new_data = String::from_utf8(data)
                            .expect("invalid data")
                            .lines()
                            .map(|line| {
                                let mut l = prefix.clone();
                                l.push_str(line);
                                l
                            })
                            .collect::<Vec<_>>()
                            .join("\n");

                        stdout
                            .write_all(new_data.as_bytes())
                            .await
                            .expect("could not write to stout");
                        stdout
                            .write_all("\n".as_bytes())
                            .await
                            .expect("could not write to stout");
                    }
                },
            };
        }
    }
}
