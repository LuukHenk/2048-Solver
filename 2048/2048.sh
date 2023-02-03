
MY_PATH="`dirname \"$0\"`"
mkdir -p -- "$data"
cargo run --manifest-path=$MY_PATH/tfe_engine/tfe/Cargo.toml -- $MY_PATH/data/results.json
sh $MY_PATH/display_latest_results.sh
