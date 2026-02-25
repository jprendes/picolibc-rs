@init-submodule:
    test -f crates/sys/picolibc/COPYING.picolibc || ( \
        git submodule update --init && \
        git -C crates/sys/picolibc sparse-checkout init --no-cone && \
        git -C crates/sys/picolibc sparse-checkout set '**' '!test/**' '!scripts/**' '!COPYING.GPL2' )

build: init-submodule
    cargo build -p picolibc-demo

run: build
    cargo run -p picolibc-demo

clippy: init-submodule
    cargo clippy --workspace -- -D warnings

fmt:
    cargo +nightly fmt --check