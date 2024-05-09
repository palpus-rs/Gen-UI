use std::path::{Path, PathBuf};

use makepad_gen_plugin::Makepad;

pub enum Target {
    Slint,
    Dioxus,
    Makepad,
}

pub enum CompilerTarget {
    Slint,
    Dioxus,
    Makepad(Option<Makepad>),
}

impl CompilerTarget {
    pub fn init<P>(&mut self, entry: &str, path: P, root: Option<&PathBuf>) -> ()
    where
        P: AsRef<Path>,
    {
        match self {
            CompilerTarget::Slint => todo!("Slint is not supported yet"),
            CompilerTarget::Dioxus => todo!("Dioxus is not supported yet"),
            CompilerTarget::Makepad(makepad) => {
                if let None = makepad {
                    let instance = Makepad::new(entry, path, root);
                    makepad.replace(instance);
                }
            }
        }
    }
}

impl From<Target> for CompilerTarget {
    fn from(value: Target) -> Self {
        match value {
            Target::Slint => CompilerTarget::Slint,
            Target::Dioxus => CompilerTarget::Dioxus,
            Target::Makepad => CompilerTarget::Makepad(None),
        }
    }
}
