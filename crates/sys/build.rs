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
    ("single-thread", "true"),
    ("enable-malloc", "true"),
    ("newlib-nano-malloc", "true"),
    ("thread-local-storage", "true"),
    ("newlib-global-errno", "true"),
    ("tests", "false"),
    ("semihost", "false"),
    ("tinystdio", "true"),
    ("io-c99-formats", "false"),
    ("io-wchar", "true"),
    ("posix-console", "true"),
    ("freestanding", "true"),
    ("picolib", "false"), // disabled as we implement our own sbrk
    ("specsdir", "spec"),
];

fn main() -> Result<()> {
    build::build(OPTIONS)
}
