#/bin/bash

cargo build --release && \
strip 'target/release/log-converter' && \
docker build . --no-cache -t 'kimstebel/log-converter:1.0.0' && \
docker run 'kimstebel/log-converter:1.0.0'