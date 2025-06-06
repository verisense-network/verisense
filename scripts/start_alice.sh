#!/usr/bin/env sh

(
sudo /data/verisense \
  --base-path /data/config \
  --chain /data/betanet.json \
  --port 30333 \
  --rpc-port 9944 \
  --rpc-cors=all \
  --validator \
  --tss-coordinator 12944 \
  2>&1 | tee -a /data/verisense.out
) &