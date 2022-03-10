use std::fs::File;
use std::io::{BufRead, BufReader, Write};
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
        let hash = match line.split_once(":") {
            Some((hash, often)) => hash,
            _ => return Err(anyhow!("invalid hash pattern"))
        };

        bloom.set(hash);
    }

    let mut bloomfile = File::open("easypwned.bloom")?;
    bloomfile.write_all(bloom.bitmap().as_ref())?;
    
    println!("done!");

    Ok(())
}