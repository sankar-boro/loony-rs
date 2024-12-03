#!/usr/bin/env sh

# Check if the number of arguments is exactly 2
if [ "$#" -eq 1 ]; then
    if [ "$1" = "--build-backend" ]; then
        echo "Building loony-axum-postgres..."
        cargo build --release
    fi
fi

USERNAME="sankar"
HOME=$HOME
ORIGINS="http://localhost:3000,http://localhost:8081,http://127.0.0.1:8081,http://10.0.2.2:8081,http://192.168.6.48:8081,http://127.0.0.1:2000"

RUST_LOG=info \
HOST=localhost \
PORT=8000 \
ORIGINS="$ORIGINS" \
SEARCH_INDEX_PATH="/home/sankar/.tantivy" \
./target/release/loony-search
