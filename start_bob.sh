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