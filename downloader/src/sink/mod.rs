pub mod stdout;
pub mod bloom;

pub enum SinkMsg {
    Data(Vec<u8>, ::tokio::sync::oneshot::Receiver<()>),
    Finish,
}