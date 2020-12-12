use crate::error::AppError;
use rand::{distributions::Uniform, Rng};

const NUM_CHAR: &[u8] = "0123456789".as_bytes();

fn generate(characters: &[u8], length: u32) -> Result<String, AppError> {
    let rng = rand::thread_rng();
    let range = Uniform::from(0..characters.len());
    let bytes: Vec<u8> = rng
        .sample_iter(range)
        .take(length as usize)
        .map(|x| characters[x]) // as char)
        .collect();
    Ok(std::str::from_utf8(&bytes)?.to_string())
}

pub fn generate_numeric(length: u32) -> Result<String, AppError> {
    generate(NUM_CHAR, length)
}
