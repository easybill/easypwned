use anyhow::Context;
use byteorder::WriteBytesExt;
use reqwest::Client;
use std::time::Duration;
use tokio::sync::mpsc::Sender;

#[derive(Debug)]
pub struct DownloaderCommanderMsgWorkResult {
    pub range: u32,
    pub prefix: String,
    pub bytes: Vec<u8>,
}

#[derive(Debug)]
pub struct DownloaderCommanderMsgWork {
    pub range: u32,
}

#[derive(Debug)]
pub enum DownloaderCommanderMsgResponse {
    AskForWork(Option<DownloaderCommanderMsgWork>),
}
#[derive(Debug)]
pub enum DownloaderCommanderMsgRequest {
    AskForWork(::tokio::sync::oneshot::Sender<Option<DownloaderCommanderMsgWork>>),
    SendWork(DownloaderCommanderMsgWorkResult),
}

#[derive(Debug)]
pub struct DownloaderHttp {
    commander: Sender<DownloaderCommanderMsgRequest>,
}

impl DownloaderHttp {
    pub fn spawn(commander: Sender<DownloaderCommanderMsgRequest>) {
        ::tokio::spawn(async move {
            (Self { commander })
                .run()
                .await
                .expect("downloader crashed");
        });
    }

    pub async fn run(&mut self) -> Result<(), ()> {
        let client = reqwest::ClientBuilder::new()
            .brotli(true)
            .gzip(true)
            .deflate(true)
            .timeout(Duration::from_secs(10))
            .tcp_keepalive(Duration::from_secs(100))
            .danger_accept_invalid_certs(true)
            .build()
            .expect("could not build client");

        loop {
            let (response_sender, response_recv) = ::tokio::sync::oneshot::channel();

            let _msg = self
                .commander
                .send(DownloaderCommanderMsgRequest::AskForWork(response_sender))
                .await;

            let mut work = match response_recv.await {
                Ok(v) => match v {
                    Some(v) => v,
                    None => {
                        // all done.
                        return Ok(());
                    }
                },
                Err(_e) => {
                    // all done. coordinator does not exist anymore.
                    return Ok(());
                }
            };

            loop {
                match self.do_work(&mut work, &client).await {
                    Ok(_) => break,
                    Err(_e) => {
                        eprint!("could not fetch work {}, retry...", work.range);
                        ::tokio::time::sleep(::tokio::time::Duration::from_secs(1)).await;
                    }
                }
            }
        }
    }

    pub fn build_hash(id: u32) -> String {
        let mut buf = vec![];
        buf.write_u32::<::byteorder::BigEndian>(id)
            .expect("invalid write u32");
        ::hex::encode(&buf[1..])[1..].to_uppercase()
    }

    pub async fn do_work(
        &mut self,
        work: &mut DownloaderCommanderMsgWork,
        client: &Client,
    ) -> Result<(), ::anyhow::Error> {
        let hash = Self::build_hash(work.range);

        let body = client
            .get(format!("https://api.pwnedpasswords.com/range/{}", hash))
            .send()
            .await
            .context("could not fetch hash")?
            .bytes()
            .await
            .context("could not decode response")?;

        match self
            .commander
            .send(DownloaderCommanderMsgRequest::SendWork(
                DownloaderCommanderMsgWorkResult {
                    range: work.range,
                    prefix: hash.to_string(),
                    bytes: body.to_vec(),
                },
            ))
            .await
        {
            Ok(_) => {}
            Err(e) => {
                eprintln!("could not send work result: {:?}", e);
                panic!("could not send work result");
            }
        }

        Ok(())
    }
}
