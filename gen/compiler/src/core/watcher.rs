use std::{path::Path, sync::mpsc::channel, time::Duration};

use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};

use crate::msg::WATCHER_INIT;

use super::log::{error, info, warn};

pub async fn init_watcher(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
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
                info(format!("{:?}", event).as_str());
            }
            Err(e) => {
                warn(e.to_string().as_str());
            }
        }
    }

    Ok(())
}
