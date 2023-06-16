use bincode::config::BigEndian;
use tokio::sync::mpsc::Sender;
use byteorder::WriteBytesExt;

#[derive(Debug)]
pub struct DownloaderCommanderMsgWorkResult {
    pub range: u32,
    pub bytes: Vec<u8>,
}

#[derive(Debug)]
pub struct DownloaderCommanderMsgWork {
    pub range : u32,
}

#[derive(Debug)]
pub enum DownloaderCommanderMsgResponse {
    AskForWork(Option<DownloaderCommanderMsgWork>)
}
#[derive(Debug)]
pub enum DownloaderCommanderMsgRequest {
    AskForWork(::tokio::sync::oneshot::Sender<Option<DownloaderCommanderMsgWork>>),
    SendWork(DownloaderCommanderMsgWorkResult)
}

#[derive(Debug)]
pub struct DownloaderHttp {
    commander: Sender<DownloaderCommanderMsgRequest>,
}

impl DownloaderHttp {

    pub fn spawn(commander: Sender<DownloaderCommanderMsgRequest>) {
        ::tokio::spawn(async move {
            (Self {commander}).run().await.expect("downloader crashed");
        });
    }

    pub async fn run(&mut self) -> Result<(), ()> {
        loop {

            let (response_sender, response_recv) = ::tokio::sync::oneshot::channel();

            let msg = self.commander.send(
                DownloaderCommanderMsgRequest::AskForWork(response_sender)
            ).await;

            let mut work = match response_recv.await {
                Ok(v) => match v {
                    Some(v) => v,
                    None => {
                        // all done.
                        return Ok(())
                    }
                },
                Err(e) => {
                    // all done. coordinator does not exist anymore.
                    return Ok(());
                }
            };

            self.do_work(&mut work).await;
        }
    }

    pub fn build_hash(id : u32) -> String {
        let mut buf = vec![];
        buf.write_u32::<::byteorder::BigEndian>(id).expect("invalid write u32");
        ::hex::encode(&buf[1..])[1..].to_uppercase()
    }

    pub async fn do_work(&mut self, work : &mut DownloaderCommanderMsgWork) {
        // println!("{}", Self::build_hash(work.range));
        self.commander.send(DownloaderCommanderMsgRequest::SendWork(
            DownloaderCommanderMsgWorkResult {
                range: work.range,
                bytes: vec![],
            }
        )).await.expect("could not send work result")
    }
}