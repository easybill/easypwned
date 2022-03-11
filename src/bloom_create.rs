use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use anyhow::{anyhow, Error};
use bloomfilter::Bloom;
use serde_derive::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct BloomWithMetadata {
    number_of_bits: u64,
    number_of_hash_functions: u32,
    sip_keys: [(u64, u64); 2],
    bloom: Vec<u8>,
}

impl BloomWithMetadata {
    pub fn to_bloom(self) -> Bloom<Vec<u8>> {
        Bloom::from_existing(
            self.bloom.as_slice(),
            self.number_of_bits,
            self.number_of_hash_functions,
            self.sip_keys
        )
    }
}

fn count_lines(path : &str) -> Result<usize, Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut i = 0;
    for raw_line in reader.lines() {
        i = i + 1;
    }

    Ok(i)
}

pub fn bloom_get() -> Result<BloomWithMetadata, Error> {

    let mut file = File::open("easypwned.bloom")?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;


    Ok(bincode::deserialize::<BloomWithMetadata>(buf.as_slice())?)
}

pub fn bloom_create() -> Result<BloomWithMetadata, Error> {
    // let path = "pwned-passwords-sha1-ordered-by-hash-v8.txt";
    let path = "pwned-passwords-sha1-ordered-by-hash-v8.txt";

    println!("counting lines ...");
    let number_of_items = count_lines(path)?;
    println!("{} lines ...", number_of_items);

    println!("creating bloom");
    let mut bloom = Bloom::new_for_fp_rate(number_of_items, 0.01);
    println!("created bloom");

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for raw_line in reader.lines() {
        let line = raw_line?;

        if line.trim() == "" {
            continue;
        }

        let hash = match line.split_once(":") {
            Some((hash, often)) => hash,
            _ => return Err(anyhow!("invalid hash pattern"))
        };

        bloom.set(hash.as_bytes());
    }

    // number_of_bits: 8120685768, number_of_hash_functions: 7, sip_keys: [(16911278473676693785, 7977664235212237539), (6990288496210676087, 3334793689188451176)]
    println!("number_of_bits: {:?}, number_of_hash_functions: {:?}, sip_keys: {:?}",
        bloom.number_of_bits(),
        bloom.number_of_hash_functions(),
        bloom.sip_keys(),
    );
    // panic!();

    let bincode_with_metadata = BloomWithMetadata {
        number_of_bits: bloom.number_of_bits(),
        number_of_hash_functions: bloom.number_of_hash_functions(),
        sip_keys: bloom.sip_keys(),
        bloom: bloom.bitmap().to_vec()
    };

    let mut bloomfile = File::create("easypwned.bloom")?;
    bloomfile.write_all(bincode::serialize(&bincode_with_metadata).expect("could not bincode").as_slice())?;

    println!("done!");

    Ok(bincode_with_metadata)
}