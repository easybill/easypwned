use crate::sink::SinkMsg;
use crate::DownloadConfig;
use ::tokio::sync::mpsc::{Receiver, Sender};
use anyhow::anyhow;
use bloomfilter::Bloom;
use rand_aes::{Aes256Ctr128, Random};
use std::thread::yield_now;
use std::time::Instant;
use tokio::io::AsyncWriteExt;
use tokio::task::JoinHandle;

const BLOOM_CAPACITY: usize = 2_100_000_000;
const BLOOM_TARGET_FALSE_POSITIVE_RATE: f64 = 0.001; // 0.1%
const VERIFY_ITERATIONS: u64 = 10_000_000;
const VERIFY_PROGRESS_INTERVAL: u64 = 1_000_000;

pub struct SinkBloom {
    config: DownloadConfig,
    recv: Receiver<SinkMsg>,
    bloom_entries: usize,
}

impl SinkBloom {
    pub fn spawn(config: DownloadConfig) -> (JoinHandle<()>, Sender<SinkMsg>) {
        let (sender, recv) = ::tokio::sync::mpsc::channel(50_000);

        let jh = ::tokio::spawn(async move {
            (Self {
                recv,
                config,
                bloom_entries: 0,
            })
            .run()
            .await
            .expect("stdout sink crashed.");
        });

        (jh, sender)
    }

    pub async fn run(&mut self) -> Result<(), ()> {
        let mut bloom: Bloom<[u8]> =
            Bloom::new_for_fp_rate(BLOOM_CAPACITY, BLOOM_TARGET_FALSE_POSITIVE_RATE)
                .expect("could not create bloom filter");

        loop {
            match self.recv.recv().await {
                None => continue,
                Some(s) => match s {
                    SinkMsg::Finish => {
                        self.finish(bloom)
                            .await
                            .expect("could not write bloom file");
                        return Ok(());
                    }
                    SinkMsg::Data(prefix, data) => {
                        self.process_data(prefix, &mut bloom, data)
                            .expect("could not parse data");
                    }
                },
            };
        }
    }

    pub async fn finish(&self, bloom: Bloom<[u8]>) -> Result<(), ::anyhow::Error> {
        eprintln!("start writing bloom filter.");

        let bloom_file_path = self
            .config
            .opt
            .sink_bloom_file
            .as_ref()
            .expect("must be given");

        let mut bloom_data_file = ::tokio::fs::File::create(bloom_file_path).await?;
        bloom_data_file.write_all(bloom.as_slice()).await?;

        eprintln!("finished writing bloom filter.");

        self.verify(bloom).await?;

        Ok(())
    }

    pub fn process_data(
        &mut self,
        prefix: String,
        bloom: &mut Bloom<[u8]>,
        data: Vec<u8>,
    ) -> Result<(), ::anyhow::Error> {
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
                }
                _ => return Err(anyhow!("invalid hash pattern")),
            };

            bloom.set(hash.as_bytes());
            self.bloom_entries += 1;
        }

        Ok(())
    }

    async fn verify(&self, bloom: Bloom<[u8]>) -> Result<(), ::anyhow::Error> {
        eprintln!("starting false positive rate verification ({VERIFY_ITERATIONS} iterations)...");

        let rng = Aes256Ctr128::from_entropy();
        let mut buf = [0u8; 512];
        let mut false_positives: u64 = 0;

        let start = Instant::now();

        for i in 1..=VERIFY_ITERATIONS {
            rng.fill_bytes(&mut buf[..]);

            if bloom.check(&buf) {
                false_positives += 1;
            }

            if i.is_multiple_of(VERIFY_PROGRESS_INTERVAL) {
                let rate = false_positives as f64 / i as f64;
                eprintln!(
                    "  [{}/{}] false positives: {}, observed rate: {:.6}",
                    i, VERIFY_ITERATIONS, false_positives, rate
                );
                yield_now()
            }
        }

        let elapsed = start.elapsed();
        let observed_rate = false_positives as f64 / VERIFY_ITERATIONS as f64;

        eprintln!();
        eprintln!("=== Verification Results ===");
        eprintln!("Iterations:         {}", VERIFY_ITERATIONS);
        eprintln!("False positives:    {}", false_positives);
        eprintln!("Observed FP rate:   {:.6}", observed_rate);
        eprintln!(
            "Expected FP rate:   {:.6}",
            BLOOM_TARGET_FALSE_POSITIVE_RATE
        );
        eprintln!(
            "Ratio (obs/exp):    {:.3}",
            observed_rate / BLOOM_TARGET_FALSE_POSITIVE_RATE
        );
        eprintln!("Test duration:      {:.2?}", elapsed);
        eprintln!(
            "Checks per second:  {:.0}",
            VERIFY_ITERATIONS as f64 / elapsed.as_secs_f64()
        );

        if self.bloom_entries > BLOOM_CAPACITY {
            eprintln!("WARNING: Bloom filter entries ({}) exceeded capacity of bloom filter. Increase the {BLOOM_CAPACITY}", self.bloom_entries);
        }

        if observed_rate <= BLOOM_TARGET_FALSE_POSITIVE_RATE * 1.1 {
            eprintln!("PASS: Observed false positive rate is within 10% of expected.");
            Ok(())
        } else {
            eprintln!("FAIL: Observed false positive rate exceeds 10% of the expected value.");
            Err(anyhow!("elevated false positive rate"))
        }
    }
}
