use crate::error::Error;
use rand::{distributions::Uniform, Rng};

const NUM_CHAR: &[u8] = "0123456789".as_bytes();

pub fn generate(characters: &[u8], length: u32) -> Result<String, Error> {
    let rng = rand::thread_rng();
    let range = Uniform::from(0..characters.len());
    let bytes: Vec<u8> = rng
        .sample_iter(range)
        .take(length as usize)
        .map(|x| characters[x]) // as char)
        .collect();
    Ok(std::str::from_utf8(&bytes)?.to_string())
}

pub fn generate_numeric(length: u32) -> Result<String, Error> {
    generate(NUM_CHAR, length)
}