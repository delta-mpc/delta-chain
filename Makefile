b:
	cargo build --release
alice:
	./target/release/frontier-template-node --no-mdns --validator --chain local --alice -d ./.local/alice \
	--port 30333 --ws-port 9944 --rpc-port 9933 \
	--node-key 0000000000000000000000000000000000000000000000000000000000000001
bob:
	./target/release/frontier-template-node --no-mdns --validator --chain local --bob -d ./.local/bob \
	--port 30334 --ws-port 9945 --rpc-port 9934 \
	--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp