use std::env;
use std::path::Path;

use anyhow::{Context, Result};

mod bindings;
mod builder;
mod runtime;

use builder::Builder;
use runtime::Sandbox;

pub fn build(options: &[(&str, &str)]) -> Result<()> {
    let mut cc = cc::Build::new();

    let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;
    let manifest_dir = Path::new(&manifest_dir);
    let out_dir = env::var("OUT_DIR")?;
    let out_dir = Path::new(&out_dir);

    // install_dir should be so that include dir is <sysroot>/usr/include so that
    // bindgen can find the headers with -isysroot=<sysroot>
    let src_dir = manifest_dir.join("picolibc");
    let build_dir = out_dir.join("build");
    let install_dir = out_dir.join("usr");
    let include_dir = install_dir.join("include");

    cc.out_dir(&build_dir.join("obj"));
    cc.warnings_into_errors(true);

    println!("cargo:rerun-if-changed={}", src_dir.display());

    let sandbox = Sandbox::new(cc.get_compiler());

    let builder = Builder::new(cc.clone());
    let libraries = builder.get_libraries();

    let mut meson = picomeson::Meson::new(sandbox, builder);

    // Add buildtype option
    let buildtype = match std::env::var("OPT_LEVEL")?.as_str() {
        "0" => "debug",
        "1" | "2" | "3" => "release",
        "s" | "z" => "minsize",
        _ => {
            println!("cargo:warning=Unknown OPT_LEVEL, defaulting to 'minsize'");
            "minsize"
        }
    };
    meson.option("buildtype", buildtype);

    // Add prefix option
    meson.option("prefix", install_dir.to_string_lossy());

    // Add user-defined options
    for def in options {
        meson.option(def.0, def.1);
    }

    meson.build(src_dir.to_string_lossy(), build_dir.to_string_lossy())?;

    // Add stub files
    let stubs = &[
        manifest_dir.join("stubs").join("clock.c"),
        manifest_dir.join("stubs").join("start.c"),
        manifest_dir.join("stubs").join("sbrk.c"),
        manifest_dir.join("stubs").join("errno.c"),
    ];

    for file in stubs {
        println!("cargo:rerun-if-changed={}", file.display());
    }

    libraries
        .borrow_mut()
        .get_mut("c")
        .context("Could not find libc")?
        .files(stubs);

    for (name, build) in libraries.borrow().iter() {
        build.compile(name);
    }

    bindings::generate(&include_dir, &out_dir.join("picolibc.rs"))?;

    Ok(())
}
