#!/usr/bin/env sh

(
sudo /data/verisense \
  --base-path /data/config \
  --chain /data/betanet.json \
  --port 30333 \
  --rpc-port 9944 \
  --validator \
  --tss-signer /ip4/10.128.0.2/tcp/12944/p2p/12D3KooWFcGs16mdf3HuNd2KMx5WYNsDyyDVz9h6Udg6WWg3CCxh \
  2>&1 | tee /data/verisense.out
)&

# ./target/release/verisense key generate-node-key --file /tmp/dave/node-key
# ./target/release/verisense \
# --base-path /tmp/dave \
# --chain local \
# --dave \
# --node-key-file /tmp/dave/node-key \
# --port 30336 \
# --rpc-port 9947 \
# --prometheus-port 9618 \
# --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
# --validator \
# --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp \
# --tss-signer /ip4/127.0.0.1/tcp/12944/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp
