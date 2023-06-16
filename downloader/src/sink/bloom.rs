use ::tokio::sync::mpsc::{Sender, Receiver};
use anyhow::anyhow;
use bloomfilter::Bloom;
use tokio::task::JoinHandle;
use crate::sink::SinkMsg;

struct SinkBloom {
    recv: Receiver<SinkMsg>
}

impl SinkBloom {
    pub fn spawn() -> (JoinHandle<()>, Sender<SinkMsg>) {
        let (sender, recv) = ::tokio::sync::mpsc::channel(1000);

        let jh = ::tokio::spawn(async move {
            (Self { recv }).run().await.expect("stdout sink crashed.");
        });

        (jh, sender)
    }

    pub async fn run(&mut self) -> Result<(), ()> {

        let mut bloom : Bloom<[u8]> = Bloom::new_for_fp_rate(700_000_000, 0.01);

        loop {
            match self.recv.recv().await {
                None => continue,
                Some(s) => match s {
                    SinkMsg::Finish => return Ok(()),
                    SinkMsg::Data(data, ok) => {
                        self.process_data(&mut bloom, data).expect("could not parse data");
                    }
                }
            };
        }
    }

    pub fn process_data(&mut self, bloom: &mut Bloom<[u8]>, data: Vec<u8>) -> Result<(), ::anyhow::Error> {
        let data_string = match String::from_utf8(data) {
            Ok(v) => v,
            Err(e) => {
                return Err(anyhow!("invalid utf8"));
            }
        };

        for line in data_string.lines() {

            if line.trim() == "" {
                continue;
            }

            let hash = match line.split_once(":") {
                Some((hash, _often)) => hash,
                _ => return Err(anyhow!("invalid hash pattern")),
            };

            bloom.set(hash.as_bytes());
        }

        Ok(())
    }
}