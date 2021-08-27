# delta-chain
The Delta Blockchain built with Substrate

# Build & Run

To build the chain, execute the following commands from the project root:

```
$ cargo build --release
```

To execute the chain, run:

```
$ ./target/release/delta-chain-node --dev
```

The node also supports to use manual seal (to produce block manually through RPC).  
This is also used by the ts-tests:

```
$ ./target/release/delta-chain-node --dev --manual-seal
```

### Docker Based Development
Building (takes 5-10 min):

```bash
docker-compose build
```

Running

```bash
docker-compose up -d
```

# Node Configuration

### Add a well-know node

`Sudo => nodeAuthorization => add_well_known_node` 

### Add a validator
1. `RPC => author => rotateKeys`(On the node that is about to become a validator)
2. `Extrinsics => session => setKeys` (From account that is about to become a validator)
3. `Sudo => validatorSet => addValidator`

### Transfer Ether to dev account
```
Extrinsics => evm => call

source: 0xcee2b721fc2fcbb3c136effec5d555c9f9c97db1
target: 0xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
input: 0x
value: 1000000000000000000000 // 1000 Ether
gas_limit: 4294967295
gas_price: 1
nonce: <empty>

```



