pub mod stdout;
pub mod bloom;

#[derive(Debug)]
pub enum SinkMsg {
    Data(String, Vec<u8>),
    Finish,
}