# Build the project
cd programs/turn_based_engine
cargo build-sbf
cd ../..

SBF_OUT_DIR=$(pwd)/target/deploy cargo nextest run
