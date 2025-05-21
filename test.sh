# Build the project
cd programs/example_program
cargo build-sbf
cd ../..

SBF_OUT_DIR=$(pwd)/target/deploy cargo nextest run
