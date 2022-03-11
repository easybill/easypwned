use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use anyhow::{anyhow, Error};
use bloomfilter::Bloom;

fn count_lines(path : &str) -> Result<usize, Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut i = 0;
    for raw_line in reader.lines() {
        i = i + 1;
    }

    Ok(i)
}

pub fn bloom_get() -> Result<Bloom<Vec<u8>>, Error> {

    let mut file = File::open("easypwned.bloom")?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;


    Ok(Bloom::from_existing(
        &buf,
        8120685768,
        7, [(16911278473676693785, 7977664235212237539), (6990288496210676087, 3334793689188451176)]
    ))

    /*
    Bloom::from_existing(
        &original.bitmap(),
        original.number_of_bits(),
        original.number_of_hash_functions(),
        original.sip_keys(),
    );
     */
}

pub fn bloom_create() -> Result<(), Error> {
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

    let mut bloomfile = File::create("easypwned.bloom")?;
    bloomfile.write_all(bloom.bitmap().as_ref())?;

    println!("done!");

    Ok(())
}