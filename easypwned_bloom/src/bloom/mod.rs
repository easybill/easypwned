use anyhow::{anyhow, Error};
use bloomfilter::Bloom;
use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct BloomWithMetadata {
    pub number_of_bits: u64,
    pub number_of_hash_functions: u32,
    pub sip_keys: [(u64, u64); 2],
    pub bloom: Vec<u8>,
}

pub type EasyBloom = Bloom<Vec<u8>>;

impl BloomWithMetadata {
    pub fn to_bloom(self) -> EasyBloom {
        Bloom::from_existing(
            self.bloom.as_slice(),
            self.number_of_bits,
            self.number_of_hash_functions,
            self.sip_keys,
        )
    }
}

pub fn bloom_get(path: &str) -> Result<BloomWithMetadata, Error> {
    let mut file = File::open(&path)?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;

    Ok(bincode::deserialize::<BloomWithMetadata>(buf.as_slice())?)
}
