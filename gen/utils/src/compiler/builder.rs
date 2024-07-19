/// # Builder trait
/// Builder trait is a trait to builder pattern
/// 
/// **Although there are few methods that need to be implemented uniformly in this trait, it exists as a specification**
pub trait Builder {
    type Target;
    /// ## Build the target
    /// cusume the builder and return the target
    fn build(self) -> Self::Target;
}