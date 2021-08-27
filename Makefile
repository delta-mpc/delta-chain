b:
	cargo build --release
alice:
	./target/release/delta-chain-node --no-mdns --validator --chain delta -d ./.local/alice \
	--port 30333 --ws-port 9944 --rpc-port 9933 \
	--node-key 2d2c7a1c5b92fc77aef5524851fc301d91a0a958bb68a9a5fe617e5a8e9f66d6
bob:
	./target/release/delta-chain-node --no-mdns --validator --chain delta -d ./.local/bob \
	--port 30334 --ws-port 9945 --rpc-port 9934 \
	--node-key 41a86336376cde451ae861f4d89e42b8e9fec9d2174a1860451f7300d02b284d \
	--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWKpxeRfkRPmCQUb9PCBVPgdo139n3g8SNAKE3Ap4V6bWy
charlie:
	./target/release/delta-chain-node --no-mdns --validator --chain local --charlie -d ./.local/charlie \
	--port 30335 --ws-port 9946 --rpc-port 9935 \
	--node-key 3a9d5b35b9fb4c42aafadeca046f6bf56107bd2579687f069b42646684b94d9e  \
	--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2
dave:
	./target/release/delta-chain-node --no-mdns --chain local -d ./.local/dave \
	--offchain-worker always --name dave \
	--port 30336 --ws-port 9947 --rpc-port 9936 \
	--node-key a99331ff4f0e0a0434a6263da0a5823ea3afcfffe590c9f3014e6cf620f2b19a   \
	--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2
spec:
	./target/release/delta-chain-node build-spec --chain delta > spec.json