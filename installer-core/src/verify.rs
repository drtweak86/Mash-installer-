use std::fs::File;
use std::io::Read;
use std::path::Path;

use anyhow::{anyhow, Context, Result};

/// Re-read a file after write and verify its content matches expectations.
///
/// Compares the full length and the first/last `SAMPLE_SIZE` bytes to detect
/// truncated or corrupted writes without hashing the entire file.
pub fn verify_file_written(path: &Path, expected_content: &[u8]) -> Result<()> {
    let mut file = File::open(path)
        .with_context(|| format!("re-opening {} for verification", path.display()))?;

    let mut actual = Vec::new();
    file.read_to_end(&mut actual)
        .with_context(|| format!("reading {} for verification", path.display()))?;

    if actual.len() != expected_content.len() {
        return Err(anyhow!(
            "verification failed for {}: expected {} bytes, found {} bytes",
            path.display(),
            expected_content.len(),
            actual.len()
        ));
    }

    const SAMPLE_SIZE: usize = 256;
    let check_len = SAMPLE_SIZE.min(expected_content.len());

    // Check head
    if actual[..check_len] != expected_content[..check_len] {
        return Err(anyhow!(
            "verification failed for {}: first {} bytes differ",
            path.display(),
            check_len
        ));
    }

    // Check tail
    if expected_content.len() > SAMPLE_SIZE {
        let tail_start = expected_content.len() - check_len;
        if actual[tail_start..] != expected_content[tail_start..] {
            return Err(anyhow!(
                "verification failed for {}: last {} bytes differ",
                path.display(),
                check_len
            ));
        }
    }

    Ok(())
}

/// Force an fsync on the given path to ensure the write survives power loss.
///
/// Critical on Raspberry Pi with SD card storage.
pub fn sync_file(path: &Path) -> Result<()> {
    let file = File::open(path).with_context(|| format!("opening {} for fsync", path.display()))?;
    file.sync_all()
        .with_context(|| format!("fsync failed for {}", path.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn verify_succeeds_for_matching_content() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.txt");
        let content = b"The forge glows with plasma runes.";
        fs::write(&path, content).unwrap();
        assert!(verify_file_written(&path, content).is_ok());
    }

    #[test]
    fn verify_fails_for_wrong_length() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.txt");
        fs::write(&path, b"short").unwrap();
        let result = verify_file_written(&path, b"much longer content");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("bytes"));
    }

    #[test]
    fn verify_fails_for_corrupted_content() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.txt");
        let original = b"correct content here";
        let corrupted = b"CORRUPT content here";
        fs::write(&path, corrupted).unwrap();
        let result = verify_file_written(&path, original);
        assert!(result.is_err());
    }

    #[test]
    fn sync_file_succeeds_on_valid_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("sync_test.txt");
        fs::write(&path, b"data to sync").unwrap();
        assert!(sync_file(&path).is_ok());
    }

    #[test]
    fn sync_file_fails_on_missing_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("nonexistent.txt");
        assert!(sync_file(&path).is_err());
    }

    #[test]
    fn verify_large_file_checks_head_and_tail() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("large.txt");
        let mut content = vec![b'A'; 1024];
        content[0] = b'X'; // distinctive head
        content[1023] = b'Z'; // distinctive tail
        fs::write(&path, &content).unwrap();
        assert!(verify_file_written(&path, &content).is_ok());

        // Corrupt the tail
        let mut corrupted = content.clone();
        corrupted[1023] = b'Y';
        fs::write(&path, &corrupted).unwrap();
        assert!(verify_file_written(&path, &content).is_err());
    }
}
