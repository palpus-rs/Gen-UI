use std::{path::{Path, PathBuf}, sync::mpsc::channel, time::Duration};

use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};

use crate::msg::WATCHER_INIT;

use super::log::{error, info, warn};

/// ## init watcher
/// init watcher to watch file change event
/// - f: compile the file , copy to src_gen and write cache 
pub async fn init_watcher<F>(path: &Path, f: F) -> Result<(), Box<dyn std::error::Error>> 
where F: Fn(&Vec<PathBuf>) -> (),
{
    let (tx, rx) = channel();

    let config = Config::default();
    config.with_poll_interval(Duration::from_secs(2));

    let mut watcher = RecommendedWatcher::new(tx, config)?;

    watcher.watch(
        path,
        RecursiveMode::Recursive,
    )?;

    info(WATCHER_INIT);

    while let Ok(event) = rx.recv() {
        match event {
            Ok(event) => {
                // check event is modify or create
                // info(format!("{:?}", event).as_str());
                if event.kind.is_modify() || event.kind.is_create() {
                    // compile the file , copy to src_gen and write cache
                    f(&event.paths);
                }
            }
            Err(e) => {
                warn(e.to_string().as_str());
            }
        }
    }

    Ok(())
}
