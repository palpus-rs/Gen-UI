use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
    process::exit,
};

use gen_converter::model::Model;
use gen_utils::{
    common::{
        msg::{APP_RUNNING, SRC_GEN_INIT},
        Source,
    },
    compiler::{CompilerImpl, Executor},
};

use tokio::runtime::Runtime;

use walkdir::WalkDir;

use crate::{copy_file, info, init_watcher, is_eq_path_exclude, Cache};

use super::{log::error, watcher::FKind};

/// ## Compile Strategy: Lazy
/// compiler will compile the file when the file is created or modified
///
/// but it will not compile the dir, only compile the file in the dir
///
/// dir will be generated after the file in the dir is compiled
pub struct Compiler {
    /// origin path is the project path
    pub origin_path: PathBuf,
    /// the path of the compiled project (default is src_gen)
    pub compiled_path: PathBuf,
    /// compile target default is makepad
    pub target: Box<dyn CompilerImpl>,
    /// exclude files or folders
    pub exclude: Vec<PathBuf>,
    /// gen_cache
    pub cache: Cache,
}

impl Compiler {
    /// ## run the compiler
    /// - run the compiler and watch the file change
    /// - compile the file when the file is created or modified
    /// - remove the compiled file|dir when the file|dir is removed
    pub fn run(&mut self) -> () {
        info(APP_RUNNING);
        let rt = Runtime::new().unwrap();
        let origin_path = self.origin_path.clone();
        let excludes = self.exclude.clone();
        rt.block_on(async {
            if let Err(e) =
                init_watcher(origin_path.as_path(), &excludes, |path, e_kind, f_kind| {
                    match e_kind {
                        notify::EventKind::Create(_) | notify::EventKind::Modify(_) => {
                            // create or modify
                            self.compile_one(path);
                        }
                        notify::EventKind::Remove(_) => {
                            // remove from cache and compiled project, after test we know, only remove need f_kind to know the file is dir or file
                            self.remove_compiled(path, f_kind);
                        }
                        _ => (),
                    }
                    // do other auxiliary work
                    let _ = self.execute_auxiliaries(Executor {
                        success: Box::new(|msg| {
                            info(msg);
                        }),
                        fail: Box::new(|e| error(e.to_string().as_str())),
                        ignore: Box::new(|| {
                            ();
                        }),
                    });
                })
                .await
            {
                // log error and stop the service
                error(e.to_string().as_str());
                return;
            }
        });
        exit(-1);
    }
    // /// set wasm
    // pub fn wasm<W>(&mut self, wasm: Box<W>) -> &mut Self
    // where
    //     W: WasmImpl,
    // {
    //     self.target.set_wasm(wasm);
    //     self
    // }
    // /// fresh wasm when the wasm file is modified
    // pub fn fresh_wasm(&mut self) -> () {
    //     if self.wasm {
    //         // close last wasm process if exist
    //         if let Some(process) = self.wasm_process.as_mut() {
    //             let _ = process.kill();
    //         }
    //         let mut super_workspace_path = self.origin_path.clone();
    //         super_workspace_path.pop();
    //         match self.target.fresh_wasm(super_workspace_path.as_path()) {
    //             Ok(cmd) => {
    //                 self.wasm_process.replace(cmd);
    //                 info(WASM_FRESH);
    //             }
    //             Err(e) => error(e.to_string().as_str()),
    //         }
    //     }
    // }
    // pub fn add_dep(&mut self, dep: RustDependence) -> &mut Self {
    //     self.dependencies.push(dep);
    //     self
    // }
    // /// set app entry name
    // pub fn entry(&mut self, entry: &str) -> &mut Self {
    //     self.entry = entry.to_string();
    //     self
    // }
    // /// set the root path of the project(which need to be excluded from the compile target)
    // pub fn root<P>(&mut self, path: P) -> &mut Self
    // where
    //     P: AsRef<Path>,
    // {
    //     // self.exclude.push(absolute_or_path(path.as_ref()));
    //     let root_path = path.as_ref().to_path_buf();
    //     // add root into cache
    //     let _ = self.cache.exists_or_insert(root_path.as_path());
    //     self.root.replace(root_path);
    //     self
    // }
    // pub fn init_compile_target(&mut self) -> () {
    //     let _ = self
    //         .target
    //         .init(&self.entry, self.origin_path.as_path(), self.root.as_ref());
    // }
    /// ## compile the project
    /// ### example
    /// ```rust
    /// use gen_compiler::app;
    ///
    /// fn main(){
    ///    let compiler = app(CompilerTarget::Makepad, "./ui");
    ///    compiler.compile();
    /// }
    /// ```
    /// ### tests
    /// - easy compile: 👌
    pub fn compile(&mut self) -> () {
        let _ = self.exist_or_create();
        info(SRC_GEN_INIT);
        // let _ = self.init_compile_target();
        let mut visited = HashSet::new();
        // after src_gen project created, get compile target and then use plugin logic to rewrite
        Compiler::loop_compile(self, &mut visited);
        // after all files compiled
        let _ = self.target.compile(self.cache.get_gen().as_ref());
        // write cache
        let _ = self.cache.write();

        let _ = self.execute_auxiliaries(Executor {
            success: Box::new(|msg| {
                info(msg);
            }),
            fail: Box::new(|e| error(e.to_string().as_str())),
            ignore: Box::new(|| {
                ();
            }),
        });
    }
    /// compile single gen / other type file
    fn compile_one<P>(&mut self, path: P) -> ()
    where
        P: AsRef<Path>,
    {
        let target_path = self.origin_path.as_path().to_path_buf();
        match (
            path.as_ref().is_file(),
            path.as_ref().to_str().unwrap().ends_with(".gen"),
        ) {
            (false, true) | (false, false) => {
                // if is dir, do nothing , use lazy compile(only dir has file, file will be compiled, dir generate after file compiled)
                return;
            }
            (true, true) => {
                self.cache
                    .exists_or_insert(path.as_ref())
                    .unwrap()
                    .modify_then(|| {
                        let model =
                            Model::new(&path.as_ref().to_path_buf(), &target_path, false).unwrap();
                        let source = model.get_special().clone();
                        // match &mut self.target {
                        //     CompilerTarget::Makepad(makepad) => {
                        //         makepad.as_mut().unwrap().add(model);
                        //     }
                        //     CompilerTarget::Slint => todo!("slint plugin not implemented yet"),
                        //     CompilerTarget::Dioxus => {
                        //         todo!("dioxus plugin not implemented yet")
                        //     }
                        // }
                        let _ = self.insert(Box::new(model));
                        // get the compiled result from target and then copy to the compiled project
                        // this step may faild (2024-05-27)
                        // let _ = self
                        //     .target
                        //     .get(&source)
                        //     .expect("node can not be found(system error)")
                        //     .compile();
                        let _ = self.get(&source).unwrap().compile();
                    });
                let _ = self.cache.write();
            }
            (true, false) => {
                // not gen file, directly copy to the compiled project
                let compiled_path =
                    Source::origin_file_without_gen(path.as_ref(), target_path.as_path());

                let _ = self
                    .cache
                    .exists_or_insert(path.as_ref())
                    .unwrap()
                    .modify_then(|| {
                        let _ = copy_file(path.as_ref(), compiled_path);
                    });
                let _ = self.cache.write();
            }
        }
        info(format!("file {:?} is compiled successfully.", path.as_ref()).as_str());
    }
    /// remove compiled file and remove cache
    fn remove_compiled<P>(&mut self, path: P, f_kind: FKind) -> ()
    where
        P: AsRef<Path>,
    {
        info(format!("{:?} is removing ...", path.as_ref()).as_str());
        // if path is dir, recursively remove all files in the dir and then remove the dir (also remove cache)

        if f_kind.is_dir() {
            // get all files in the dir
            let compiled_path =
                Source::origin_dir_to_compiled(self.origin_path.as_path(), path.as_ref());
            dbg!(compiled_path.as_path());
            let _ = fs::remove_dir_all(compiled_path.as_path()).expect("remove dir failed");
            // remove from cache
            let _ = self.cache.remove_all(path.as_ref());
        } else {
            let compiled_path = if path.as_ref().to_str().unwrap().ends_with(".gen") {
                Source::origin_file_to_compiled(path.as_ref(), self.origin_path.as_path())
            } else {
                Source::origin_file_without_gen(path.as_ref(), self.origin_path.as_path())
            };

            if compiled_path.as_path().exists() {
                // remove compiled file
                let _ = fs::remove_file(compiled_path.as_path()).unwrap();
                // remove cache
                let _ = self.cache.remove(path);
            }
        }
        let _ = self.cache.write();
    }
    fn loop_compile(compiler: &mut Compiler, visited: &mut HashSet<PathBuf>) {
        // Convert to absolute path
        // let target_path = target.as_ref().canonicalize().unwrap();
        let target_path = compiler.origin_path.as_path().to_path_buf();
        if !visited.insert(target_path.clone()) {
            return;
        }

        for item in WalkDir::new(target_path.as_path())
            .into_iter()
            .filter_map(|d| d.ok())
        {
            let source_path = item.path();
            // check if the file or folder is in the exclude list, if true, skip it
            if compiler
                .exclude
                .iter()
                .any(|uncompiled_item| is_eq_path_exclude(source_path, uncompiled_item.as_path()))
            {
                continue;
            }

            match (
                source_path.is_file(),
                source_path.to_str().unwrap().ends_with(".gen"),
            ) {
                (false, true) | (false, false) => {
                    // is dir should loop compile again
                    Compiler::loop_compile(compiler, visited);
                }
                (true, true) => {
                    // is gen file, use target compiler to compile then copy to the compiled project
                    // compiler
                    //     .cache
                    //     .exists_or_insert(&source_path)
                    //     .unwrap()
                    //     .modify_then(|| {
                    //         dbg!(&source_path);
                    //         let model = Model::new(&source_path.to_path_buf(), &target_path, false)
                    //             .unwrap();
                    //         match &mut compiler.target {
                    //             CompilerTarget::Makepad(makepad) => {
                    //                 makepad.as_mut().unwrap().add(model);
                    //             }
                    //             CompilerTarget::Slint => todo!("slint plugin not implemented yet"),
                    //             CompilerTarget::Dioxus => {
                    //                 todo!("dioxus plugin not implemented yet")
                    //             }
                    //         }
                    //     });
                    let model =
                        Model::new(&source_path.to_path_buf(), &target_path, false).unwrap();
                    // match &mut compiler.target {
                    //     CompilerTarget::Makepad(makepad) => {
                    //         makepad.as_mut().unwrap().add(model);
                    //     }
                    //     CompilerTarget::Slint => todo!("slint plugin not implemented yet"),
                    //     CompilerTarget::Dioxus => {
                    //         todo!("dioxus plugin not implemented yet")
                    //     }
                    // }
                    let _ = compiler.insert(Box::new(model));
                }
                (true, false) => {
                    // is file but not gen file, directly copy to the compiled project
                    // get the compiled path
                    let compiled_path = Source::origin_file_without_gen(source_path, &target_path);
                    // check and insert into cache
                    let _ = compiler
                        .cache
                        .exists_or_insert(source_path)
                        .unwrap()
                        .modify_then(|| {
                            let _ = copy_file(source_path, compiled_path);
                        });
                }
            }
        }
    }
    /// ## add exclude file or folder
    /// path root is the project root path
    pub fn push_exclude<P>(&mut self, path: P) -> &mut Self
    where
        P: AsRef<Path>,
    {
        let path = self.origin_path.join(path.as_ref());
        self.exclude.push(path);
        self
    }
}

impl CompilerImpl for Compiler {
    fn execute_auxiliaries(&mut self, executor: Executor) -> () {
        let _ = self.target.execute_auxiliaries(executor);
    }

    fn exist_or_create(&self) -> () {
        let _ = self.target.exist_or_create();
    }

    fn compile(&mut self, gen_files: Option<&Vec<&PathBuf>>) -> () {
        let _ = self.target.compile(gen_files);
    }

    fn insert(&mut self, node: Box<dyn std::any::Any>) -> () {
        let _ = self.target.insert(node);
    }

    fn get(
        &self,
        key: &gen_utils::common::Source,
    ) -> Option<Box<dyn gen_utils::compiler::ModelNodeImpl>> {
        self.target.get(key)
    }
}
