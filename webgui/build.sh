#!/bin/bash
(
cd "${0%/*}"
cargo web deploy
cp target/deploy/* server/
cd server
echo "Starting the server... use"
trap 'kill %1' 2
python3 server.py &
cygstart chrome "http://localhost:8080"
wait
)
