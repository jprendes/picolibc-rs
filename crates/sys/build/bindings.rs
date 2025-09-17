use std::path::Path;

use anyhow::Context as _;

pub fn generate(inc_path: &Path, out_path: &Path) -> anyhow::Result<()> {
    // inc path is <sysroot>/usr/include
    let sysroot_path = inc_path
        .parent()
        .context("invalid sysroot")?
        .parent()
        .context("invalid sysroot")?;

    let mut bindings = bindgen::builder()
        .use_core()
        .derive_copy(true)
        .derive_debug(true)
        .derive_default(true)
        .derive_eq(true)
        .derive_hash(true)
        .derive_ord(true)
        .generate_comments(true)
        .generate_cstr(true)
        .clang_arg(format!("-isysroot{}", sysroot_path.to_string_lossy()))
        .clang_arg("-D_POSIX_MONOTONIC_CLOCK=1")
        .clang_arg("-D_POSIX_C_SOURCE=200809L");

    let mut all_headers = String::from("#pragma once;\n");

    for header in glob::glob(&format!("{}/**/*.h", inc_path.display()))? {
        let header = header?;

        let rel_path = header.strip_prefix(inc_path)?;
        if skip_header_bindgen(bindings.clone(), &header) {
            continue;
        }

        all_headers.push_str(&format!("#include <{}>\n", rel_path.to_string_lossy()));
        //bindings = bindings.header(header.to_string_lossy());
    }

    bindings = bindings.header_contents("picolibc.h", &all_headers);

    // Write the generated bindings to an output file.
    bindings.generate()?.write_to_file(out_path)?;

    Ok(())
}

fn skip_header_bindgen(bindings: bindgen::Builder, header: &Path) -> bool {
    // some headers are not intended to be included directly, or are just stubs
    // that error out, and we should skip them
    bindings
        .header_contents("test.h", &format!("#include \"{}\"", header.display()))
        .generate()
        .is_err()
}
