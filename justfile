run: build
    ./target/bootstrap/freight build
    ./target/debug/freight help

build:
    rm -rf target
    mkdir -p target/bootstrap_stage0
    # Build crate dependencies
    rustc --edition 2021 --crate-name=freight \
        --crate-type=lib \
        --out-dir=target/bootstrap \
        src/lib.rs
    # Create the executable
    rustc --edition 2021 --crate-name=freight \
        --crate-type=bin \
        --extern freight \
        -L target/bootstrap \
        --out-dir=target/bootstrap \
        src/main.rs
