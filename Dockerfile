FROM paritytech/ci-linux:production as builder
WORKDIR /delta
COPY . .
RUN cargo build --release

FROM paritytech/ci-linux:production
WORKDIR /delta
COPY --from=builder /delta/target/release/delta-chain-node ./node

# 30333 for p2p traffic
# 9933 for RPC call
# 9944 for Websocket
# 9615 for Prometheus (metrics)
EXPOSE 30333 9933 9944 9615

ENTRYPOINT ["./node", "--no-mdns", "--chain", "delta"]
