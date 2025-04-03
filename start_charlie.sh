./target/release/verisense key generate-node-key --file /tmp/charlie/node-key
./target/release/verisense \
--base-path /tmp/charlie \
--chain local \
--charlie \
--node-key-file /tmp/charlie/node-key \
--port 30333 \
--rpc-port 9945 \
--prometheus-port 9615 \
--telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
--validator \
--bootnodes /ip4/10.128.0.2/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp \
--tss-signer /ip4/10.128.0.2/tcp/12944/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp
