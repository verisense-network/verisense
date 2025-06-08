./target/release/verisense key generate-node-key --file /tmp/bob/node-key
./target/release/verisense \
--base-path /tmp/bob \
--chain local \
--bob \
--node-key-file /tmp/bob/node-key \
--port 30334 \
--rpc-port 9945 \
--prometheus-port 9616 \
--telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
--validator \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp