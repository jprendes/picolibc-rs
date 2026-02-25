use anyhow::Result;

#[path = "build/mod.rs"]
mod build;

const OPTIONS: &[(&str, &str)] = &[
    ("multilib", "false"),
    ("format-default", "double"),
    ("posix-console", "false"),
    ("printf-aliases", "false"),
    ("picocrt", "false"),
    ("initfini-array", "true"),
    ("single-thread", "false"),
    ("stdio-locking", "true"),
    ("enable-malloc", "true"),
    ("thread-local-storage", "true"),
    ("newlib-global-errno", "false"),
    ("errno-function", "__errno_location"),
    ("tests", "false"),
    ("semihost", "false"),
    ("io-c99-formats", "false"),
    ("io-wchar", "true"),
    ("posix-console", "true"),
    ("freestanding", "true"),
    ("os-linux", "false"),
    ("specsdir", "spec"),
];

fn main() -> Result<()> {
    build::build(OPTIONS)
}
