use anyhow::Result;

#[path = "build/mod.rs"]
mod build;

const OPTIONS: &[(&str, &str)] = &[
    ("multilib", "false"),
    ("format-default", "double"),
    ("posix-console", "false"),
    ("printf-aliases", "false"),
    ("atomic-ungetc", "true"),
    ("fast-bufio", "true"),
    ("stdio-locking", "true"),
    ("fast-strcmp", "true"),
    ("picocrt", "false"),
    ("initfini-array", "true"),
    ("single-thread", "false"),
    ("enable-malloc", "true"),
    ("malloc-small-bucket", "1024"),
    ("thread-local-storage", "true"),
    ("thread-local-storage-api", "false"),
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
    ("want-math-errno", "true"),
];

fn main() -> Result<()> {
    build::build(OPTIONS)
}
