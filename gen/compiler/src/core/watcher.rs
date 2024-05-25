use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::mpsc::channel,
    time::Duration,
};

use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};

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
    F: FnMut(&Path, &notify::EventKind, FKind) -> (),
{
    let (tx, rx) = channel();

    let config = Config::default();
    config.with_poll_interval(Duration::from_secs(2));

    let mut watcher = RecommendedWatcher::new(tx, config)?;
    let mut fs_state = get_current_state(path)?;

    watcher.watch(path, RecursiveMode::Recursive)?;

    info(WATCHER_INIT);

    while let Ok(event) = rx.recv() {
        match event {
            Ok(event) => {
                // check event is modify or create
                // info(format!("{:?}", event).as_str());
                // if match event.kind {
                //     notify::EventKind::Create(_)
                //     | notify::EventKind::Modify(_)
                //     | notify::EventKind::Remove(_) => true,
                //     _ => false,
                // } {
                //     // compile the file , copy to src_gen and write cache
                //     // attention: exclude some files
                //     if !excludes.contains(&event.paths[0]) {
                //         f(&event.paths[0], &event.kind);
                //     }
                // }
                if !excludes.contains(&event.paths[0]) {
                    if let Some(kind) = match_kind(&event, &mut fs_state) {
                        f(&event.paths[0], &event.kind, kind);
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

/// specify the kind of file|dir
#[derive(Clone, Copy, Debug)]
pub enum FKind {
    File,
    Dir,
    Unknown,
}

impl FKind {
    pub fn is_dir(&self) -> bool {
        matches!(self, FKind::Dir)
    }
    pub fn is_file(&self) -> bool {
        matches!(self, FKind::File)
    }
}

fn match_kind(event: &Event, fs_state: &mut HashMap<PathBuf, FKind>) -> Option<FKind> {
    let path = event.paths[0].as_path();
    match event.kind {
        notify::EventKind::Create(_) | notify::EventKind::Modify(_) => {
            let kind = get_kind(path);
            fs_state.insert(path.to_path_buf(), kind);
            Some(kind)
        }
        notify::EventKind::Remove(_) => {
            let kind = fs_state.get(path).copied();
            fs_state.remove(event.paths[0].as_path());
            kind
        }
        _ => None,
    }
}

fn get_current_state<P>(path: P) -> Result<HashMap<PathBuf, FKind>, Box<dyn std::error::Error>>
where
    P: AsRef<Path>,
{
    let mut state = HashMap::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let kind = get_kind(&path);
        match kind {
            FKind::Dir => {
                let dir_state = get_current_state(&path)?;
                state.extend(dir_state)
            }
            _ => (),
        }
        state.insert(path, kind);
    }
    Ok(state)
}

fn get_kind<P>(path: P) -> FKind
where
    P: AsRef<Path>,
{
    let path = path.as_ref();

    if path.is_file() {
        FKind::File
    } else if path.is_dir() {
        FKind::Dir
    } else {
        FKind::Unknown
    }
}
