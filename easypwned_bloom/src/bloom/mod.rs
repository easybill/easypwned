use anyhow::Error;
use bloomfilter::Bloom;
use std::fs::File;
use std::io::Read;

pub type EasyBloom = Bloom<[u8]>;

pub fn bloom_get(path: &str) -> Result<EasyBloom, Error> {
    let mut file = File::open(path)?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;

    let bloom = Bloom::from_bytes(buf).map_err(|e| anyhow::anyhow!(e))?;
    Ok(bloom)
}
