use anyhow::{anyhow, Error};
use bloomfilter::Bloom;
use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct BloomWithMetadata {
    number_of_bits: u64,
    number_of_hash_functions: u32,
    sip_keys: [(u64, u64); 2],
    bloom: Vec<u8>,
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

pub fn bloom_create(bloom_file: &str, password_file: &str) -> Result<BloomWithMetadata, Error> {
    // let path = "pwned-passwords-sha1-ordered-by-hash-v8.txt";

    if Path::new(bloom_file).exists() {
        return Err(anyhow!("bloomfile already exists"));
    }

    println!("creating bloom");
    let mut bloom = Bloom::new_for_fp_rate(700_000_000, 0.01);
    println!("created bloom");

    let file = File::open(password_file)?;
    let reader = BufReader::new(file);

    for raw_line in reader.lines() {
        let line = raw_line?;

        if line.trim() == "" {
            continue;
        }

        let hash = match line.split_once(":") {
            Some((hash, _often)) => hash,
            _ => return Err(anyhow!("invalid hash pattern")),
        };

        bloom.set(hash.as_bytes());
    }

    // number_of_bits: 8120685768, number_of_hash_functions: 7, sip_keys: [(16911278473676693785, 7977664235212237539), (6990288496210676087, 3334793689188451176)]
    println!(
        "number_of_bits: {:?}, number_of_hash_functions: {:?}, sip_keys: {:?}",
        bloom.number_of_bits(),
        bloom.number_of_hash_functions(),
        bloom.sip_keys(),
    );
    // panic!();

    let bincode_with_metadata = BloomWithMetadata {
        number_of_bits: bloom.number_of_bits(),
        number_of_hash_functions: bloom.number_of_hash_functions(),
        sip_keys: bloom.sip_keys(),
        bloom: bloom.bitmap().to_vec(),
    };

    let mut bloomfile = File::create(bloom_file)?;
    bloomfile.write_all(
        bincode::serialize(&bincode_with_metadata)
            .expect("could not bincode")
            .as_slice(),
    )?;

    println!("done!");

    Ok(bincode_with_metadata)
}
