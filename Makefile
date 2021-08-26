b:
	cargo build --release
alice:
	./target/release/frontier-template-node --no-mdns --validator --chain local --alice -d ./.local/alice \
	--port 30333 --ws-port 9944 --rpc-port 9933 \
	--node-key c12b6d18942f5ee8528c8e2baf4e147b5c5c18710926ea492d09cbd9f6c9f82a
bob:
	./target/release/frontier-template-node --no-mdns --validator --chain local --bob -d ./.local/bob \
	--port 30334 --ws-port 9945 --rpc-port 9934 \
	--node-key 6ce3be907dbcabf20a9a5a60a712b4256a54196000a8ed4050d352bc113f8c58 \
	--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2
charlie:
	./target/release/frontier-template-node --no-mdns --chain local -d ./.local/charlie \
	--offchain-worker always --name charlie \
	--port 30335 --ws-port 9946 --rpc-port 9935 \
	--node-key 3a9d5b35b9fb4c42aafadeca046f6bf56107bd2579687f069b42646684b94d9e  \
	--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2
dave:
	./target/release/frontier-template-node --no-mdns --chain local -d ./.local/dave \
	--offchain-worker always --name dave \
	--port 30336 --ws-port 9947 --rpc-port 9936 \
	--node-key a99331ff4f0e0a0434a6263da0a5823ea3afcfffe590c9f3014e6cf620f2b19a   \
	--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2