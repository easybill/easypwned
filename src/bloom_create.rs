use std::fs::File;
use std::io::BufReader;

pub fn bloom_create() -> Result<(), ()> {
    let file = File::open("foo.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}