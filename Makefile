ifneq (,$(wildcard ./.env))
    include .env
    export
endif

b:
	cargo build --release
alice:
	./target/release/delta-chain-node --no-mdns --validator --chain delta -d ./.local/alice \
	--port 30333 --ws-port 9944 --rpc-port 9933 \
	--node-key ${AliceNodeKey}
bob:
	./target/release/delta-chain-node --no-mdns --validator --chain delta -d ./.local/bob \
	--port 30334 --ws-port 9945 --rpc-port 9934 \
	--node-key ${BobNodeKey} \
	--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWKpxeRfkRPmCQUb9PCBVPgdo139n3g8SNAKE3Ap4V6bWy
spec:
	./target/release/delta-chain-node build-spec --chain delta > spec.json