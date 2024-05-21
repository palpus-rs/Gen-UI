use std::{
    path::{Path, PathBuf},
    sync::mpsc::channel,
    time::Duration,
};

use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};

use crate::msg::WATCHER_INIT;

use super::log::{error, info, warn};

/// ## init watcher
/// init watcher to watch file change event
/// - f: compile the file , copy to src_gen and write cache
pub async fn init_watcher<F>(
    path: &Path,
    excludes: &Vec<PathBuf>,
    mut f: F,
) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnMut(&Path, &notify::EventKind) -> (),
{
    let (tx, rx) = channel();

    let config = Config::default();
    config.with_poll_interval(Duration::from_secs(2));

    let mut watcher = RecommendedWatcher::new(tx, config)?;

    watcher.watch(path, RecursiveMode::Recursive)?;

    info(WATCHER_INIT);

    while let Ok(event) = rx.recv() {
        match event {
            Ok(event) => {
                // check event is modify or create
                // info(format!("{:?}", event).as_str());
                if match event.kind {
                    notify::EventKind::Create(_)
                    | notify::EventKind::Modify(_)
                    | notify::EventKind::Remove(_) => true,
                    _ => false,
                } {
                    // compile the file , copy to src_gen and write cache
                    // attention: exclude some files
                    if !excludes.contains(&event.paths[0]) {
                        f(&event.paths[0], &event.kind);
                    }
                }
            }
            Err(e) => {
                warn(e.to_string().as_str());
            }
        }
    }

    Ok(())
}
