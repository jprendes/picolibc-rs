@init-submodule:
    test -f crates/sys/picolibc/COPYING.picolibc || \
        git submodule update --init && \
        git -C crates/sys/picolibc sparse-checkout init --no-cone && \
        git -C crates/sys/picolibc sparse-checkout set '**' '!test/**' '!scripts/**' '!COPYING.GPL2'

run: init-submodule
    cargo run -p picolibc-demo

clippy:
    cargo clippy --workspace -- -D warnings

fmt:
    cargo +nightly fmt --check