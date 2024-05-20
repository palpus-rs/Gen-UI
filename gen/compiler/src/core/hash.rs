use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

use sha2::{Digest, Sha256};

/// ## calculate the hash of a file
/// calc hash use sha256
pub fn calc_hash<P>(path: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let mut file = File::options().read(true).open(path.as_ref())?;
    let mut hasher = Sha256::new();

    let mut buf = [0; 1024];
    // loop read
    loop {
        let b_read = file.read(&mut buf)?;
        if b_read == 0 {
            break;
        }
        // update the hasher with the read buffer
        hasher.update(&buf[..b_read]);
    }

    // calc hash
    let hash_value = hasher.finalize();

    Ok(format!("{:x}", hash_value))
}
