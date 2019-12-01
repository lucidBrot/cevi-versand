#!/bin/bash
(
cd "${0%/*}"
cargo web deploy || { exit 1; }
cp target/deploy/* server/
cd server
echo "Starting the server..."
trap 'kill %1' 2
python3 server.py &
cygstart chrome "http://localhost:8080"
wait
)
