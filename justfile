run: build
    ./target/bootstrap_stage0/freight_stage0
    ./target/bootstrap_stage1/freight_stage1 help

build:
    mkdir -p target/bootstrap_stage0
    # Build crate dependencies
    rustc --edition 2021 --crate-name=freight \
        --crate-type=lib \
        --out-dir=target/bootstrap_stage0 \
        src/lib.rs
    # Create the executable
    rustc --edition 2021 --crate-name=freight_stage0 \
        --crate-type=bin \
        --extern freight \
        -L target/bootstrap_stage0 \
        --out-dir=target/bootstrap_stage0 \
        src/main.rs
