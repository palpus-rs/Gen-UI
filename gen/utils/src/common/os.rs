/// # Os Type
/// Os type is used to determine the current operating system.
/// 
/// use **Os::current** to get the current operating system.
pub enum Os{
    Windows,
    Linux,
    Mac,
    Other,
}

impl Os{
    /// ## Get Current Operating System
    pub fn current() -> Self{
        let os = std::env::consts::OS;
        match os{
            "windows" => Os::Windows,
            "linux" => Os::Linux,
            "macos" => Os::Mac,
            _ => Os::Other,
        }
    }
}