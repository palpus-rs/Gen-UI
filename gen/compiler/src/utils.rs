use std::{fs, path::Path, thread, time::Duration};

pub fn copy_file<P, Q>(from: P, to: Q) -> ()
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    // Extract the directory part from the compiled_path
    if let Some(parent_dir) = to.as_ref().parent() {
        // Check if the directory exists, if not, create it
        if !parent_dir.exists() {
            // Create the directory and any necessary parent directories
            fs::create_dir_all(parent_dir).expect("Failed to create directory");
        }
    }

    // Copy the file from source_path to compiled_path
    // fs::copy(from, to).expect("Failed to copy file to compiled project");
    let _ = copy_with_retries(from, to, 5, Duration::from_millis(500)).unwrap();
}

fn copy_with_retries<P, Q>(
    from: P,
    to: Q,
    max_attempts: usize,
    delay: Duration,
) -> std::io::Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let mut attempts = 0;
    loop {
        match fs::copy(from.as_ref(), to.as_ref()) {
            Ok(_) => return Ok(()),
            Err(_) if attempts < max_attempts => {
                attempts += 1;
                thread::sleep(delay);
            }
            Err(e) => return Err(e),
        }
    }
}
