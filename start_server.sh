#!/bin/bash

# Starts a local web-server that serves the contents of the `doc/` folder,

echo "ensuring basic-http-server is installed…"
cargo install basic-http-server

echo "starting server…"
echo "serving at http://localhost:8787"

(cd docs && basic-http-server --addr 127.0.0.1:8787 .)
# (cd docs && python3 -m http.server 8888 --bind 127.0.0.1)
