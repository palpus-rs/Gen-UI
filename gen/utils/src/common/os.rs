pub enum Os{
    Windows,
    Linux,
    Mac,
    Other,
}

impl Os{
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