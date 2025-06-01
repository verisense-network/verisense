./target/release/verisense key generate-node-key --file /tmp/dave/node-key
./target/release/verisense \
--base-path /tmp/dave \
--dave \
--validator \
--chain local \
--node-key-file /tmp/dave/node-key \
--port 30336 \
--rpc-port 9947 \
--prometheus-port 9618 \
--telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp