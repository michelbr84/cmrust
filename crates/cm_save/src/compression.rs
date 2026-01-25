//! Compression utilities.

use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use crate::errors::SaveError;

/// Write gzip compressed data to file.
pub fn write_gzip<P: AsRef<Path>>(path: P, data: &[u8]) -> Result<(), SaveError> {
    // Create parent directories if needed
    if let Some(parent) = path.as_ref().parent() {
        std::fs::create_dir_all(parent)?;
    }

    let file = File::create(path)?;
    let mut encoder = GzEncoder::new(file, Compression::default());
    encoder.write_all(data)?;
    encoder.finish()?;
    Ok(())
}

/// Read gzip compressed data from file.
pub fn read_gzip<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, SaveError> {
    let file = File::open(path)?;
    let mut decoder = GzDecoder::new(file);
    let mut data = Vec::new();
    decoder.read_to_end(&mut data)?;
    Ok(data)
}

/// Compress data in memory.
pub fn compress(data: &[u8]) -> Result<Vec<u8>, SaveError> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data)?;
    Ok(encoder.finish()?)
}

/// Decompress data in memory.
pub fn decompress(data: &[u8]) -> Result<Vec<u8>, SaveError> {
    let mut decoder = GzDecoder::new(data);
    let mut output = Vec::new();
    decoder.read_to_end(&mut output)?;
    Ok(output)
}
