#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileState{
    Unchanged,
    Modified,
    Created,
    Deleted,
}