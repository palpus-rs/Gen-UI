/// ## File state enum
/// which should be used to represent the state of a file
///
/// from notify::EventKind, this enum may change in the future if needed
/// > todo!("change to `struct FileState(EventKind)`")
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileState {
    Unchanged,
    Modified,
    Created,
    Deleted,
}

impl FileState {
    /// match state if is modified or created then do then function
    ///
    /// else do nothing
    pub fn modify_then<F>(&self, f: F) -> ()
    where
        F: FnOnce() -> (),
    {
        match self {
            FileState::Modified | FileState::Created => f(),
            _ => (),
        }
    }
    pub fn then<F>(&self, f: F) -> ()
    where
        F: FnOnce(&Self) -> (),
    {
        f(&self);
    }
}
