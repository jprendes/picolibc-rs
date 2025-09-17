use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::rc::Rc;

use cc::Build;
use picomeson::builder::{self, ConfigureFile};

pub struct Builder {
    cc: Build,
    pub libraries: Rc<RefCell<HashMap<String, Build>>>,
}

impl Builder {
    pub fn new(cc: Build) -> Self {
        let libraries = Rc::default();
        Self { cc, libraries }
    }
}

impl Builder {
    pub fn get_libraries(&self) -> Rc<RefCell<HashMap<String, Build>>> {
        self.libraries.clone()
    }
}

impl builder::Builder for Builder {
    fn configure_file(&self, file: &ConfigureFile) {
        let build_dir = Path::new(file.build_dir.as_ref());
        let filename = Path::new(file.filename.as_ref());
        let install_dir = Path::new(file.install_dir.as_ref());

        fs::create_dir_all(build_dir).unwrap();
        fs::write(build_dir.join(filename), &file.content).unwrap();

        if file.install {
            fs::create_dir_all(install_dir).unwrap();
            fs::copy(build_dir.join(filename), install_dir.join(filename)).unwrap();
        }
    }

    fn install_headers(
        &self,
        install_dir: &picomeson::path::Path,
        headers: &[picomeson::path::Path],
    ) {
        let install_dir = Path::new(install_dir.as_ref());
        fs::create_dir_all(install_dir).unwrap();
        for header in headers {
            let header = Path::new(header.as_ref());
            let dest = install_dir.join(header.file_name().unwrap());
            fs::copy(header, dest).unwrap();
        }
    }

    fn build_executable(&self, target: &builder::BuildTarget) {
        println!("cargo:info=Skipping executable {}", target.filename);
    }

    fn build_static_library(&self, target: &builder::BuildTarget) {
        let filename = &target.filename;

        if !target.install {
            println!("cargo:info=Skipping non-installable library {filename}");
            return;
        }

        if is_empty(&target.sources) {
            println!("cargo:info=Skipping empty library {filename}");
            return;
        }

        let mut cc = self.cc.clone();

        cc.files(target.sources.iter().map(|p| Path::new(p.as_ref())));
        cc.includes(target.include_dirs.iter().map(|p| Path::new(p.as_ref())));
        cc.flags(&target.flags);

        self.libraries.borrow_mut().insert(target.name.clone(), cc);
    }
}

fn is_empty(sources: &[picomeson::path::Path]) -> bool {
    sources.is_empty() || (sources.len() == 1 && sources[0].filename() == "empty.c")
}
