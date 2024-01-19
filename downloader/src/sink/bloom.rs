use ::tokio::sync::mpsc::{Sender, Receiver};
use anyhow::anyhow;
use bloomfilter::Bloom;
use tokio::io::AsyncWriteExt;
use tokio::task::JoinHandle;
use easypwned_bloom::bloom::BloomWithMetadata;
use crate::DownloadConfig;
use crate::sink::SinkMsg;

pub struct SinkBloom {
    config : DownloadConfig,
    recv: Receiver<SinkMsg>
}

impl SinkBloom {
    pub fn spawn(config : DownloadConfig) -> (JoinHandle<()>, Sender<SinkMsg>) {
        let (sender, recv) = ::tokio::sync::mpsc::channel(50_000);

        let jh = ::tokio::spawn(async move {
            (Self { recv, config }).run().await.expect("stdout sink crashed.");
        });

        (jh, sender)
    }

    pub async fn run(&mut self) -> Result<(), ()> {

        let mut bloom : Bloom<[u8]> = Bloom::new_for_fp_rate(1_000_000_000, 0.01);

        loop {
            match self.recv.recv().await {
                None => continue,
                Some(s) => match s {
                    SinkMsg::Finish => {
                        self.finish(&mut bloom).await.expect("could not write bloom file");
                        return Ok(())
                    },
                    SinkMsg::Data(prefix, data) => {
                        self.process_data(prefix, &mut bloom, data).expect("could not parse data");
                    }
                }
            };
        }
    }

    pub async fn finish(&self, bloom: &mut Bloom<[u8]>) -> Result<(), ::anyhow::Error> {

        eprintln!("start writing bloom filter.");

        let bincode_with_metadata = BloomWithMetadata {
            number_of_bits: bloom.number_of_bits(),
            number_of_hash_functions: bloom.number_of_hash_functions(),
            sip_keys: bloom.sip_keys(),
            bloom: bloom.bitmap().to_vec(),
        };

        let bloom_file = self.config.opt.sink_bloom_file.as_ref().expect("must be given");

        let mut bloomfile = ::tokio::fs::File::create(bloom_file).await?;
        bloomfile.write_all(
            bincode::serialize(&bincode_with_metadata)
                .expect("could not bincode")
                .as_slice(),
        ).await?;

        eprintln!("finished writing bloom filter.");

        Ok(())
    }

    pub fn process_data(&mut self, prefix: String, bloom: &mut Bloom<[u8]>, data: Vec<u8>) -> Result<(), ::anyhow::Error> {
        let data_string = match String::from_utf8(data) {
            Ok(v) => v,
            Err(_e) => {
                return Err(anyhow!("invalid utf8"));
            }
        };

        for line in data_string.lines() {

            if line.trim() == "" {
                continue;
            }

            let hash = match line.split_once(":") {
                Some((hash, _often)) => {
                    let mut s = prefix.clone();
                    s.push_str(hash);
                    s
                },
                _ => return Err(anyhow!("invalid hash pattern")),
            };

            bloom.set(hash.as_bytes());
        }

        Ok(())
    }
}