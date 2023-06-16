pub mod stdout;
pub mod bloom;

#[derive(Debug)]
pub enum SinkMsg {
    Data(Vec<u8>),
    Finish,
}