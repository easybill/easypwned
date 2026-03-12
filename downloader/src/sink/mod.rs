pub mod bloom;
pub mod stdout;

#[derive(Debug)]
pub enum SinkMsg {
    Data(String, Vec<u8>),
    Finish,
}
