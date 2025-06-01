./target/release/verisense key generate-node-key --file /tmp/charlie/node-key
./target/release/verisense \
--base-path /tmp/charlie \
--chain local \
--charlie \
--node-key-file /tmp/charlie/node-key \
--port 30335 \
--rpc-port 9946 \
--prometheus-port 9617 \
--telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
--validator \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp