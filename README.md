# linera_logger

remember to use wsl for everything if u are on windows! caution: needs like 30 GB including linera-protocol

```
git clone -b master https://github.com/toshinari123/linera_logger
cd linera_logger/logger
cargo build --release --target wasm32-unknown-unknown
cd ../logging_fungible
cargo build --release --target wasm32-unknown-unknown
```
we will use multiple terminals, denoted by [terminal].

[local network]:
```
git clone https://github.com/linera-io/linera-protocol.git
cd linera-protocol
git checkout 8d2ad58366e780f2667bbc572684ea8f409d1544
cargo install --path linera-service
./scripts/run_local.sh
```
[linera commandline]:
```
cd linera-protocol
export LINERA_WALLET="$(realpath target/debug/wallet.json)"
export LINERA_STORAGE="rocksdb:$(dirname "$LINERA_WALLET")/linera.db"
export LINERA_WALLET_2="$(realpath target/debug/wallet_2.json)"
export LINERA_STORAGE_2="rocksdb:$(dirname "$LINERA_WALLET_2")/linera_2.db"
cd ../linera_logger
linera --wallet "$LINERA_WALLET" --storage "$LINERA_STORAGE" \
publish-bytecode logger/target/wasm32-unknown-unknown/release/logger_{contract,service}.wasm
linera --wallet "$LINERA_WALLET" --storage "$LINERA_STORAGE" \
publish-bytecode logging_fungible/target/wasm32-unknown-unknown/release/logging_fungible_{contract,service}.wasm
linera --wallet "$LINERA_WALLET" --storage "$LINERA_STORAGE" \
create-application e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65010000000000000001000000
linera --wallet "$LINERA_WALLET" --storage "$LINERA_STORAGE" wallet show
```
the green chain (default chain of the wallet) should have 6 as next block height. Try closing some tabs / applications if there are fewer.
now copy the Owner field to the right of the green chain (paste where it says XXX below)

[linera commandline]:
```
linera --wallet "$LINERA_WALLET" --storage "$LINERA_STORAGE" \
create-application e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65030000000000000000000000 \
--json-argument '{"accounts":{"User:XXX": "10000."}}' \
--json-parameters '{"logger": "e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65010000000000000001000000e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65050000000000000000000000"}' \
--required-application-ids e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65010000000000000001000000e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65050000000000000000000000
```

[linera service 1]:
```
cd linera-protocol
export LINERA_WALLET="$(realpath target/debug/wallet.json)"
export LINERA_STORAGE="rocksdb:$(dirname "$LINERA_WALLET")/linera.db"
linera --wallet "$LINERA_WALLET" --storage "$LINERA_STORAGE" service --port 8080
```

[frontend]:
```
cd linera_logger/logging_fungible/web-frontend
npm install
npm start
```
go to `http://localhost:3000/e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65030000000000000000000000e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65070000000000000000000000?owner=XXX&port=8080` where XXX is the previous owner ID. You should see 10000 in your balance.

[linera commandline]:
```
linera --wallet "$LINERA_WALLET_2" --storage "$LINERA_STORAGE_2" wallet show
```
copy the owner field right to the green (only) chain to somewhere. 

[linera service 2]:
```
cd linera-protocol
export LINERA_WALLET_2="$(realpath target/debug/wallet_2.json)"
export LINERA_STORAGE_2="rocksdb:$(dirname "$LINERA_WALLET_2")/linera_2.db"
linera --wallet "$LINERA_WALLET_2" --storage "$LINERA_STORAGE_2" service --port 8081
```
Open another tab `http://localhost:3000/e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65030000000000000000000000e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65070000000000000000000000?owner=XXX2&port=8081` where XXX2 is the one you just copied.

It should say failed to pull balance because the application is only deployed when a message is sent (after you transfer some money).

Go back to the first tab opened (where it says 10000): enter chain id 1db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d03, which is the green chain in the second wallet and enter owner XXX2 and transfer some money, lets say 100.